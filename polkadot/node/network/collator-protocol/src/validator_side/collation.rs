// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Primitives for tracking collations-related data.
//!
//! Usually a path of collations is as follows:
//!    1. First, collation must be advertised by collator.
//!    2. If the advertisement was accepted, it's queued for fetch (per relay parent).
//!    3. Once it's requested, the collation is said to be Pending.
//!    4. Pending collation becomes Fetched once received, we send it to backing for validation.
//!    5. If it turns to be invalid or async backing allows seconding another candidate, carry on
//!       with the next advertisement, otherwise we're done with this relay parent.
//!
//!    ┌──────────────────────────────────────────┐
//!    └─▶Advertised ─▶ Pending ─▶ Fetched ─▶ Validated

use std::{
	collections::{BTreeMap, VecDeque},
	future::Future,
	pin::Pin,
	task::Poll,
};

use futures::{future::BoxFuture, FutureExt};
use polkadot_node_network_protocol::{
	peer_set::CollationVersion,
	request_response::{outgoing::RequestError, v1 as request_v1, OutgoingResult},
	PeerId,
};
use polkadot_node_primitives::PoV;
use polkadot_node_subsystem::jaeger;
use polkadot_node_subsystem_util::{
	metrics::prometheus::prometheus::HistogramTimer, runtime::ProspectiveParachainsMode,
};
use polkadot_primitives::{
	CandidateHash, CandidateReceipt, CollatorId, Hash, HeadData, Id as ParaId,
	PersistedValidationData,
};
use tokio_util::sync::CancellationToken;

use crate::{error::SecondingError, LOG_TARGET};

use super::GroupAssignments;

/// Candidate supplied with a para head it's built on top of.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct ProspectiveCandidate {
	/// Candidate hash.
	pub candidate_hash: CandidateHash,
	/// Parent head-data hash as supplied in advertisement.
	pub parent_head_data_hash: Hash,
}

impl ProspectiveCandidate {
	pub fn candidate_hash(&self) -> CandidateHash {
		self.candidate_hash
	}
}

/// Identifier of a fetched collation.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct FetchedCollation {
	/// Candidate's relay parent.
	pub relay_parent: Hash,
	/// Parachain id.
	pub para_id: ParaId,
	/// Candidate hash.
	pub candidate_hash: CandidateHash,
	/// Id of the collator the collation was fetched from.
	pub collator_id: CollatorId,
}

impl From<&CandidateReceipt<Hash>> for FetchedCollation {
	fn from(receipt: &CandidateReceipt<Hash>) -> Self {
		let descriptor = receipt.descriptor();
		Self {
			relay_parent: descriptor.relay_parent,
			para_id: descriptor.para_id,
			candidate_hash: receipt.hash(),
			collator_id: descriptor.collator.clone(),
		}
	}
}

/// Identifier of a collation being requested.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct PendingCollation {
	/// Candidate's relay parent.
	pub relay_parent: Hash,
	/// Parachain id.
	pub para_id: ParaId,
	/// Peer that advertised this collation.
	pub peer_id: PeerId,
	/// Optional candidate hash and parent head-data hash if were
	/// supplied in advertisement.
	pub prospective_candidate: Option<ProspectiveCandidate>,
	/// Hash of the candidate's commitments.
	pub commitments_hash: Option<Hash>,
}

impl PendingCollation {
	pub fn new(
		relay_parent: Hash,
		para_id: ParaId,
		peer_id: &PeerId,
		prospective_candidate: Option<ProspectiveCandidate>,
	) -> Self {
		Self {
			relay_parent,
			para_id,
			peer_id: *peer_id,
			prospective_candidate,
			commitments_hash: None,
		}
	}
}

/// An identifier for a fetched collation that was blocked from being seconded because we don't have
/// access to the parent's HeadData. Can be retried once the candidate outputting this head data is
/// seconded.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BlockedCollationId {
	/// Para id.
	pub para_id: ParaId,
	/// Hash of the parent head data.
	pub parent_head_data_hash: Hash,
}

/// Performs a sanity check between advertised and fetched collations.
///
/// Since the persisted validation data is constructed using the advertised
/// parent head data hash, the latter doesn't require an additional check.
pub fn fetched_collation_sanity_check(
	advertised: &PendingCollation,
	fetched: &CandidateReceipt,
	persisted_validation_data: &PersistedValidationData,
	maybe_parent_head_and_hash: Option<(HeadData, Hash)>,
) -> Result<(), SecondingError> {
	if persisted_validation_data.hash() != fetched.descriptor().persisted_validation_data_hash {
		Err(SecondingError::PersistedValidationDataMismatch)
	} else if advertised
		.prospective_candidate
		.map_or(false, |pc| pc.candidate_hash() != fetched.hash())
	{
		Err(SecondingError::CandidateHashMismatch)
	} else if maybe_parent_head_and_hash.map_or(false, |(head, hash)| head.hash() != hash) {
		Err(SecondingError::ParentHeadDataMismatch)
	} else {
		Ok(())
	}
}

/// Identifier for a requested collation and the respective collator that advertised it.
#[derive(Debug, Clone)]
pub struct CollationEvent {
	/// Collator id.
	pub collator_id: CollatorId,
	/// The network protocol version the collator is using.
	pub collator_protocol_version: CollationVersion,
	/// The requested collation data.
	pub pending_collation: PendingCollation,
}

/// Fetched collation data.
#[derive(Debug, Clone)]
pub struct PendingCollationFetch {
	/// Collation identifier.
	pub collation_event: CollationEvent,
	/// Candidate receipt.
	pub candidate_receipt: CandidateReceipt,
	/// Proof of validity.
	pub pov: PoV,
	/// Optional parachain parent head data.
	/// Only needed for elastic scaling.
	pub maybe_parent_head_data: Option<HeadData>,
}

/// The status of the collations in [`CollationsPerRelayParent`].
#[derive(Debug, Clone, Copy)]
pub enum CollationStatus {
	/// We are waiting for a collation to be advertised to us.
	Waiting,
	/// We are currently fetching a collation.
	Fetching,
	/// We are waiting that a collation is being validated.
	WaitingOnValidation,
	/// We have seconded a collation.
	Seconded,
}

impl Default for CollationStatus {
	fn default() -> Self {
		Self::Waiting
	}
}

impl CollationStatus {
	/// Downgrades to `Waiting`, but only if `self != Seconded`.
	fn back_to_waiting(&mut self, relay_parent_mode: ProspectiveParachainsMode) {
		match self {
			Self::Seconded =>
				if relay_parent_mode.is_enabled() {
					// With async backing enabled it's allowed to
					// second more candidates.
					*self = Self::Waiting
				},
			_ => *self = Self::Waiting,
		}
	}
}

/// Information about collations per relay parent.
pub struct Collations {
	/// What is the current status in regards to a collation for this relay parent?
	pub status: CollationStatus,
	/// Collator we're fetching from, optionally which candidate was requested.
	///
	/// This is the currently last started fetch, which did not exceed `MAX_UNSHARED_DOWNLOAD_TIME`
	/// yet.
	pub fetching_from: Option<(CollatorId, Option<CandidateHash>)>,
	/// Collation that were advertised to us, but we did not yet fetch. Grouped by `ParaId`.
	waiting_queue: BTreeMap<ParaId, VecDeque<(PendingCollation, CollatorId)>>,
	/// How many collations have been seconded.
	pub seconded_count: usize,
	/// What collations were fetched so far for this relay parent.
	fetched_per_para: BTreeMap<ParaId, usize>,
	// Claims per `ParaId` for the assigned core at the relay parent. This information is obtained
	// from the claim queue.
	claims_per_para: BTreeMap<ParaId, usize>,
}

impl Collations {
	pub(super) fn new(assignments: &Vec<ParaId>) -> Self {
		let mut claims_per_para = BTreeMap::new();
		for para_id in assignments {
			*claims_per_para.entry(*para_id).or_default() += 1;
		}

		Self {
			status: Default::default(),
			fetching_from: None,
			waiting_queue: Default::default(),
			seconded_count: 0,
			fetched_per_para: Default::default(),
			claims_per_para,
		}
	}

	/// Note a seconded collation for a given para.
	pub(super) fn note_seconded(&mut self) {
		self.seconded_count += 1
	}

	// Note a collation which has been successfully fetched.
	pub(super) fn note_fetched(&mut self, para_id: ParaId) {
		*self.fetched_per_para.entry(para_id).or_default() += 1
	}

	/// Returns the next collation to fetch from the `waiting_queue`.
	///
	/// This will reset the status back to `Waiting` using [`CollationStatus::back_to_waiting`].
	///
	/// Returns `Some(_)` if there is any collation to fetch, the `status` is not `Seconded` and
	/// the passed in `finished_one` is the currently `waiting_collation`.
	pub(super) fn get_next_collation_to_fetch(
		&mut self,
		finished_one: &(CollatorId, Option<CandidateHash>),
		relay_parent_mode: ProspectiveParachainsMode,
		assignments: &GroupAssignments,
	) -> Option<(PendingCollation, CollatorId)> {
		// If finished one does not match waiting_collation, then we already dequeued another fetch
		// to replace it.
		if let Some((collator_id, maybe_candidate_hash)) = self.fetching_from.as_ref() {
			// If a candidate hash was saved previously, `finished_one` must include this too.
			if collator_id != &finished_one.0 &&
				maybe_candidate_hash.map_or(true, |hash| Some(&hash) != finished_one.1.as_ref())
			{
				gum::trace!(
					target: LOG_TARGET,
					waiting_collation = ?self.fetching_from,
					?finished_one,
					"Not proceeding to the next collation - has already been done."
				);
				return None
			}
		}
		self.status.back_to_waiting(relay_parent_mode);

		match self.status {
			// We don't need to fetch any other collation when we already have seconded one.
			CollationStatus::Seconded => None,
			CollationStatus::Waiting => self.pick_a_collation_to_fetch(&assignments.current),
			CollationStatus::WaitingOnValidation | CollationStatus::Fetching =>
				unreachable!("We have reset the status above!"),
		}
	}

	/// Checks if another collation can be accepted. There are two limits:
	/// 1. The number of collations that can be seconded.
	/// 2. The number of collations that can be fetched per parachain. This is based on the number
	///    of entries in the claim queue.
	pub(super) fn is_collations_limit_reached(
		&self,
		relay_parent_mode: ProspectiveParachainsMode,
		para_id: ParaId,
	) -> bool {
		let seconded_limit =
			if let ProspectiveParachainsMode::Enabled { max_candidate_depth, .. } =
				relay_parent_mode
			{
				max_candidate_depth + 1
			} else {
				1
			};

		let respected_per_para_limit =
			self.claims_per_para.get(&para_id).copied().unwrap_or_default() >=
				self.fetched_per_para.get(&para_id).copied().unwrap_or_default();

		let respected_seconding_limit = self.seconded_count < seconded_limit;

		gum::trace!(
			target: LOG_TARGET,
			?para_id,
			claims_per_para=?self.claims_per_para,
			fetched_per_para=?self.fetched_per_para,
			?seconded_limit,
			?respected_per_para_limit,
			?respected_seconding_limit,
			"is_collations_limit_reached"
		);

		!(respected_seconding_limit && respected_per_para_limit)
	}

	/// Adds a new collation to the waiting queue for the relay parent. This function doesn't
	/// perform any limits check. The caller (`enqueue_collation`) should assure that the collation
	/// can be enqueued.
	pub(super) fn add_to_waiting_queue(&mut self, collation: (PendingCollation, CollatorId)) {
		self.waiting_queue.entry(collation.0.para_id).or_default().push_back(collation);
	}

	/// Picks a collation to fetch from the waiting queue.
	/// When fetching collations we need to ensure that each parachain has got a fair core time
	/// share depending on its assignments in the claim queue. This means that the number of
	/// collations fetched per parachain should ideally be equal to (but not bigger than) the number
	/// of claims for the particular parachain in the claim queue.
	///
	/// To achieve this each parachain with at an entry in the `waiting_queue` has got a score
	/// calculated by dividing the number of fetched collations by the number of entries in the
	/// claim queue. Lower the score means higher fetching priority. Note that if a parachain hasn't
	/// got anything fetched at this relay parent it will have score 0 which means highest priority.
	///
	/// If two parachains has got the same score the one which is earlier in the claim queue will be
	/// picked.
	fn pick_a_collation_to_fetch(
		&mut self,
		claims: &Vec<ParaId>,
	) -> Option<(PendingCollation, CollatorId)> {
		gum::trace!(
			target: LOG_TARGET,
			waiting_queue = ?self.waiting_queue,
			claim_queue = ?claims,
			"Pick a collation to fetch."
		);

		// Find the parachain(s) with the lowest score.
		let mut lowest_score = None;
		for (para_id, collations) in &mut self.waiting_queue {
			let para_score = self
				.fetched_per_para
				.get(para_id)
				.copied()
				.unwrap_or_default()
				.saturating_div(self.claims_per_para.get(para_id).copied().unwrap_or_default());

			// skip empty queues
			if collations.is_empty() {
				continue
			}

			match lowest_score {
				Some((score, _)) if para_score < score =>
					lowest_score = Some((para_score, vec![(para_id, collations)])),
				Some((_, ref mut paras)) => {
					paras.push((para_id, collations));
				},
				None => lowest_score = Some((para_score, vec![(para_id, collations)])),
			}
		}

		if let Some((_, mut lowest_score)) = lowest_score {
			for claim in claims {
				if let Some((_, collations)) = lowest_score.iter_mut().find(|(id, _)| *id == claim)
				{
					match collations.pop_front() {
						Some(collation) => return Some(collation),
						None => {
							unreachable!("Collation can't be empty!")
						},
					}
				}
			}
			unreachable!("All entries in waiting_queue should also be in claim queue")
		} else {
			None
		}
	}
}

// Any error that can occur when awaiting a collation fetch response.
#[derive(Debug, thiserror::Error)]
pub(super) enum CollationFetchError {
	#[error("Future was cancelled.")]
	Cancelled,
	#[error("{0}")]
	Request(#[from] RequestError),
}

/// Future that concludes when the collator has responded to our collation fetch request
/// or the request was cancelled by the validator.
pub(super) struct CollationFetchRequest {
	/// Info about the requested collation.
	pub pending_collation: PendingCollation,
	/// Collator id.
	pub collator_id: CollatorId,
	/// The network protocol version the collator is using.
	pub collator_protocol_version: CollationVersion,
	/// Responses from collator.
	pub from_collator: BoxFuture<'static, OutgoingResult<request_v1::CollationFetchingResponse>>,
	/// Handle used for checking if this request was cancelled.
	pub cancellation_token: CancellationToken,
	/// A jaeger span corresponding to the lifetime of the request.
	pub span: Option<jaeger::Span>,
	/// A metric histogram for the lifetime of the request
	pub _lifetime_timer: Option<HistogramTimer>,
}

impl Future for CollationFetchRequest {
	type Output = (
		CollationEvent,
		std::result::Result<request_v1::CollationFetchingResponse, CollationFetchError>,
	);

	fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
		// First check if this fetch request was cancelled.
		let cancelled = match std::pin::pin!(self.cancellation_token.cancelled()).poll(cx) {
			Poll::Ready(()) => true,
			Poll::Pending => false,
		};

		if cancelled {
			self.span.as_mut().map(|s| s.add_string_tag("success", "false"));
			return Poll::Ready((
				CollationEvent {
					collator_protocol_version: self.collator_protocol_version,
					collator_id: self.collator_id.clone(),
					pending_collation: self.pending_collation,
				},
				Err(CollationFetchError::Cancelled),
			))
		}

		let res = self.from_collator.poll_unpin(cx).map(|res| {
			(
				CollationEvent {
					collator_protocol_version: self.collator_protocol_version,
					collator_id: self.collator_id.clone(),
					pending_collation: self.pending_collation,
				},
				res.map_err(CollationFetchError::Request),
			)
		});

		match &res {
			Poll::Ready((_, Ok(_))) => {
				self.span.as_mut().map(|s| s.add_string_tag("success", "true"));
			},
			Poll::Ready((_, Err(_))) => {
				self.span.as_mut().map(|s| s.add_string_tag("success", "false"));
			},
			_ => {},
		};

		res
	}
}
