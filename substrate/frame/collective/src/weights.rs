// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-11-07, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-vcatxqpx-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/production/substrate-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_collective
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./substrate/frame/collective/src/weights.rs
// --header=./substrate/HEADER-APACHE2
// --template=./substrate/.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_collective`.
pub trait WeightInfo {
	fn set_members(m: u32, n: u32, p: u32, ) -> Weight;
	fn execute(b: u32, m: u32, ) -> Weight;
	fn propose_execute(b: u32, m: u32, ) -> Weight;
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight;
	fn vote(m: u32, ) -> Weight;
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight;
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight;
	fn close_disapproved(m: u32, p: u32, ) -> Weight;
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight;
	fn disapprove_proposal(p: u32, ) -> Weight;
	fn kill(d: u32, p: u32, ) -> Weight;
	fn release_proposal_cost() -> Weight;
}

/// Weights for `pallet_collective` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Council::Members` (r:1 w:1)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:0)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:100 w:100)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:0 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (3232 ±0) + p * (3190 ±0)`
		//  Estimated: `15927 + m * (1967 ±23) + p * (4332 ±23)`
		// Minimum execution time: 17_192_000 picoseconds.
		Weight::from_parts(17_416_000, 15927)
			// Standard Error: 63_444
			.saturating_add(Weight::from_parts(4_632_745, 0).saturating_mul(m.into()))
			// Standard Error: 63_444
			.saturating_add(Weight::from_parts(9_093_127, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_parts(0, 1967).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 4332).saturating_mul(p.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `TxPause::PausedCalls` (r:1 w:0)
	/// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `413 + m * (32 ±0)`
		//  Estimated: `3997 + m * (32 ±0)`
		// Minimum execution time: 21_850_000 picoseconds.
		Weight::from_parts(21_309_450, 3997)
			// Standard Error: 33
			.saturating_add(Weight::from_parts(1_432, 0).saturating_mul(b.into()))
			// Standard Error: 340
			.saturating_add(Weight::from_parts(13_168, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:0)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `TxPause::PausedCalls` (r:1 w:0)
	/// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `413 + m * (32 ±0)`
		//  Estimated: `3997 + m * (32 ±0)`
		// Minimum execution time: 23_981_000 picoseconds.
		Weight::from_parts(23_492_161, 3997)
			// Standard Error: 36
			.saturating_add(Weight::from_parts(1_472, 0).saturating_mul(b.into()))
			// Standard Error: 373
			.saturating_add(Weight::from_parts(20_636, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(355), added: 2830, mode: `MaxEncodedLen`)
	/// Storage: `Council::ProposalCount` (r:1 w:1)
	/// Proof: `Council::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::CostOf` (r:0 w:1)
	/// Proof: `Council::CostOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `651 + m * (32 ±0) + p * (36 ±0)`
		//  Estimated: `4024 + m * (33 ±0) + p * (36 ±0)`
		// Minimum execution time: 45_378_000 picoseconds.
		Weight::from_parts(66_281_905, 4024)
			// Standard Error: 341
			.saturating_add(Weight::from_parts(4_312, 0).saturating_mul(b.into()))
			// Standard Error: 3_562
			.saturating_add(Weight::from_parts(46_592, 0).saturating_mul(m.into()))
			// Standard Error: 3_517
			.saturating_add(Weight::from_parts(233_300, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
			.saturating_add(Weight::from_parts(0, 33).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1044 + m * (64 ±0)`
		//  Estimated: `4508 + m * (64 ±0)`
		// Minimum execution time: 28_924_000 picoseconds.
		Weight::from_parts(30_120_019, 4508)
			// Standard Error: 799
			.saturating_add(Weight::from_parts(39_255, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:0 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `633 + m * (64 ±0) + p * (36 ±0)`
		//  Estimated: `4075 + m * (65 ±0) + p * (36 ±0)`
		// Minimum execution time: 27_217_000 picoseconds.
		Weight::from_parts(31_792_776, 4075)
			// Standard Error: 2_393
			.saturating_add(Weight::from_parts(38_494, 0).saturating_mul(m.into()))
			// Standard Error: 2_334
			.saturating_add(Weight::from_parts(212_263, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 65).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `TxPause::PausedCalls` (r:1 w:0)
	/// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1080 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `4393 + b * (1 ±0) + m * (66 ±0) + p * (40 ±0)`
		// Minimum execution time: 47_899_000 picoseconds.
		Weight::from_parts(53_458_227, 4393)
			// Standard Error: 212
			.saturating_add(Weight::from_parts(2_156, 0).saturating_mul(b.into()))
			// Standard Error: 2_247
			.saturating_add(Weight::from_parts(21_907, 0).saturating_mul(m.into()))
			// Standard Error: 2_190
			.saturating_add(Weight::from_parts(228_381, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 66).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(p.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:1 w:0)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:0 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `653 + m * (64 ±0) + p * (36 ±0)`
		//  Estimated: `4095 + m * (65 ±0) + p * (36 ±0)`
		// Minimum execution time: 29_904_000 picoseconds.
		Weight::from_parts(36_341_915, 4095)
			// Standard Error: 1_937
			.saturating_add(Weight::from_parts(36_726, 0).saturating_mul(m.into()))
			// Standard Error: 1_888
			.saturating_add(Weight::from_parts(193_831, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 65).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:1 w:0)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `TxPause::PausedCalls` (r:1 w:0)
	/// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1100 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `4413 + b * (1 ±0) + m * (66 ±0) + p * (40 ±0)`
		// Minimum execution time: 50_775_000 picoseconds.
		Weight::from_parts(54_920_831, 4413)
			// Standard Error: 204
			.saturating_add(Weight::from_parts(3_415, 0).saturating_mul(b.into()))
			// Standard Error: 2_161
			.saturating_add(Weight::from_parts(24_143, 0).saturating_mul(m.into()))
			// Standard Error: 2_107
			.saturating_add(Weight::from_parts(228_205, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 66).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(p.into()))
	}
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:0 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `425 + p * (32 ±0)`
		//  Estimated: `1910 + p * (32 ±0)`
		// Minimum execution time: 14_543_000 picoseconds.
		Weight::from_parts(16_937_918, 1910)
			// Standard Error: 1_329
			.saturating_add(Weight::from_parts(169_566, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(p.into()))
	}
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::CostOf` (r:1 w:1)
	/// Proof: `Council::CostOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(355), added: 2830, mode: `MaxEncodedLen`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `d` is `[0, 1]`.
	/// The range of component `p` is `[1, 100]`.
	fn kill(d: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1896 + d * (212 ±0) + p * (41 ±0)`
		//  Estimated: `5205 + d * (1910 ±14) + p * (43 ±0)`
		// Minimum execution time: 22_315_000 picoseconds.
		Weight::from_parts(25_210_530, 5205)
			// Standard Error: 396_551
			.saturating_add(Weight::from_parts(33_466_156, 0).saturating_mul(d.into()))
			// Standard Error: 6_141
			.saturating_add(Weight::from_parts(294_900, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(d.into())))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(d.into())))
			.saturating_add(Weight::from_parts(0, 1910).saturating_mul(d.into()))
			.saturating_add(Weight::from_parts(0, 43).saturating_mul(p.into()))
	}
	/// Storage: `Council::ProposalOf` (r:1 w:0)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::CostOf` (r:1 w:1)
	/// Proof: `Council::CostOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(355), added: 2830, mode: `MaxEncodedLen`)
	fn release_proposal_cost() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1997`
		//  Estimated: `5462`
		// Minimum execution time: 69_024_000 picoseconds.
		Weight::from_parts(70_732_000, 5462)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Council::Members` (r:1 w:1)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:0)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:100 w:100)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:0 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (3232 ±0) + p * (3190 ±0)`
		//  Estimated: `15927 + m * (1967 ±23) + p * (4332 ±23)`
		// Minimum execution time: 17_192_000 picoseconds.
		Weight::from_parts(17_416_000, 15927)
			// Standard Error: 63_444
			.saturating_add(Weight::from_parts(4_632_745, 0).saturating_mul(m.into()))
			// Standard Error: 63_444
			.saturating_add(Weight::from_parts(9_093_127, 0).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_parts(0, 1967).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 4332).saturating_mul(p.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `TxPause::PausedCalls` (r:1 w:0)
	/// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `413 + m * (32 ±0)`
		//  Estimated: `3997 + m * (32 ±0)`
		// Minimum execution time: 21_850_000 picoseconds.
		Weight::from_parts(21_309_450, 3997)
			// Standard Error: 33
			.saturating_add(Weight::from_parts(1_432, 0).saturating_mul(b.into()))
			// Standard Error: 340
			.saturating_add(Weight::from_parts(13_168, 0).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:0)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `TxPause::PausedCalls` (r:1 w:0)
	/// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `413 + m * (32 ±0)`
		//  Estimated: `3997 + m * (32 ±0)`
		// Minimum execution time: 23_981_000 picoseconds.
		Weight::from_parts(23_492_161, 3997)
			// Standard Error: 36
			.saturating_add(Weight::from_parts(1_472, 0).saturating_mul(b.into()))
			// Standard Error: 373
			.saturating_add(Weight::from_parts(20_636, 0).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(355), added: 2830, mode: `MaxEncodedLen`)
	/// Storage: `Council::ProposalCount` (r:1 w:1)
	/// Proof: `Council::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::CostOf` (r:0 w:1)
	/// Proof: `Council::CostOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `651 + m * (32 ±0) + p * (36 ±0)`
		//  Estimated: `4024 + m * (33 ±0) + p * (36 ±0)`
		// Minimum execution time: 45_378_000 picoseconds.
		Weight::from_parts(66_281_905, 4024)
			// Standard Error: 341
			.saturating_add(Weight::from_parts(4_312, 0).saturating_mul(b.into()))
			// Standard Error: 3_562
			.saturating_add(Weight::from_parts(46_592, 0).saturating_mul(m.into()))
			// Standard Error: 3_517
			.saturating_add(Weight::from_parts(233_300, 0).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
			.saturating_add(Weight::from_parts(0, 33).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1044 + m * (64 ±0)`
		//  Estimated: `4508 + m * (64 ±0)`
		// Minimum execution time: 28_924_000 picoseconds.
		Weight::from_parts(30_120_019, 4508)
			// Standard Error: 799
			.saturating_add(Weight::from_parts(39_255, 0).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:0 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `633 + m * (64 ±0) + p * (36 ±0)`
		//  Estimated: `4075 + m * (65 ±0) + p * (36 ±0)`
		// Minimum execution time: 27_217_000 picoseconds.
		Weight::from_parts(31_792_776, 4075)
			// Standard Error: 2_393
			.saturating_add(Weight::from_parts(38_494, 0).saturating_mul(m.into()))
			// Standard Error: 2_334
			.saturating_add(Weight::from_parts(212_263, 0).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 65).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `TxPause::PausedCalls` (r:1 w:0)
	/// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1080 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `4393 + b * (1 ±0) + m * (66 ±0) + p * (40 ±0)`
		// Minimum execution time: 47_899_000 picoseconds.
		Weight::from_parts(53_458_227, 4393)
			// Standard Error: 212
			.saturating_add(Weight::from_parts(2_156, 0).saturating_mul(b.into()))
			// Standard Error: 2_247
			.saturating_add(Weight::from_parts(21_907, 0).saturating_mul(m.into()))
			// Standard Error: 2_190
			.saturating_add(Weight::from_parts(228_381, 0).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 66).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(p.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:1 w:0)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:0 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `653 + m * (64 ±0) + p * (36 ±0)`
		//  Estimated: `4095 + m * (65 ±0) + p * (36 ±0)`
		// Minimum execution time: 29_904_000 picoseconds.
		Weight::from_parts(36_341_915, 4095)
			// Standard Error: 1_937
			.saturating_add(Weight::from_parts(36_726, 0).saturating_mul(m.into()))
			// Standard Error: 1_888
			.saturating_add(Weight::from_parts(193_831, 0).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 65).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:1 w:0)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `TxPause::PausedCalls` (r:1 w:0)
	/// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1100 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `4413 + b * (1 ±0) + m * (66 ±0) + p * (40 ±0)`
		// Minimum execution time: 50_775_000 picoseconds.
		Weight::from_parts(54_920_831, 4413)
			// Standard Error: 204
			.saturating_add(Weight::from_parts(3_415, 0).saturating_mul(b.into()))
			// Standard Error: 2_161
			.saturating_add(Weight::from_parts(24_143, 0).saturating_mul(m.into()))
			// Standard Error: 2_107
			.saturating_add(Weight::from_parts(228_205, 0).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 66).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(p.into()))
	}
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:0 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `425 + p * (32 ±0)`
		//  Estimated: `1910 + p * (32 ±0)`
		// Minimum execution time: 14_543_000 picoseconds.
		Weight::from_parts(16_937_918, 1910)
			// Standard Error: 1_329
			.saturating_add(Weight::from_parts(169_566, 0).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(p.into()))
	}
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::CostOf` (r:1 w:1)
	/// Proof: `Council::CostOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(355), added: 2830, mode: `MaxEncodedLen`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `d` is `[0, 1]`.
	/// The range of component `p` is `[1, 100]`.
	fn kill(d: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1896 + d * (212 ±0) + p * (41 ±0)`
		//  Estimated: `5205 + d * (1910 ±14) + p * (43 ±0)`
		// Minimum execution time: 22_315_000 picoseconds.
		Weight::from_parts(25_210_530, 5205)
			// Standard Error: 396_551
			.saturating_add(Weight::from_parts(33_466_156, 0).saturating_mul(d.into()))
			// Standard Error: 6_141
			.saturating_add(Weight::from_parts(294_900, 0).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(d.into())))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
			.saturating_add(RocksDbWeight::get().writes((3_u64).saturating_mul(d.into())))
			.saturating_add(Weight::from_parts(0, 1910).saturating_mul(d.into()))
			.saturating_add(Weight::from_parts(0, 43).saturating_mul(p.into()))
	}
	/// Storage: `Council::ProposalOf` (r:1 w:0)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::CostOf` (r:1 w:1)
	/// Proof: `Council::CostOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(355), added: 2830, mode: `MaxEncodedLen`)
	fn release_proposal_cost() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1997`
		//  Estimated: `5462`
		// Minimum execution time: 69_024_000 picoseconds.
		Weight::from_parts(70_732_000, 5462)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}
