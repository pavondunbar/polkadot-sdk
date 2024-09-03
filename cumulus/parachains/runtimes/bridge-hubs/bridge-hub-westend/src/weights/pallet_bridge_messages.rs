// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_bridge_messages`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-15, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-696hpswk-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("bridge-hub-westend-dev")`, DB CACHE: 1024

// Executed Command:
// target/production/polkadot-parachain
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/builds/parity/mirrors/polkadot-sdk/.git/.artifacts/bench.json
// --pallet=pallet_bridge_messages
// --chain=bridge-hub-westend-dev
// --header=./cumulus/file_header.txt
// --output=./cumulus/parachains/runtimes/bridge-hubs/bridge-hub-westend/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_bridge_messages`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge_messages::WeightInfo for WeightInfo<T> {
	/// Storage: `BridgeRococoMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeRococoMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeRococoParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeRococoMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49209), added: 51684, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubRococo::LaneToBridge` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubRococo::LaneToBridge` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubRococo::Bridges` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubRococo::Bridges` (`max_values`: None, `max_size`: Some(1918), added: 4393, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn receive_single_message_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `701`
		//  Estimated: `52674`
		// Minimum execution time: 62_015_000 picoseconds.
		Weight::from_parts(63_891_000, 0)
			.saturating_add(Weight::from_parts(0, 52674))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgeRococoMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeRococoMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeRococoParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeRococoMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49209), added: 51684, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubRococo::LaneToBridge` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubRococo::LaneToBridge` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubRococo::Bridges` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubRococo::Bridges` (`max_values`: None, `max_size`: Some(1918), added: 4393, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 4076]`.
	fn receive_n_messages_proof(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `701`
		//  Estimated: `52674`
		// Minimum execution time: 62_034_000 picoseconds.
		Weight::from_parts(63_355_000, 0)
			.saturating_add(Weight::from_parts(0, 52674))
			// Standard Error: 8_231
			.saturating_add(Weight::from_parts(14_096_117, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgeRococoMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeRococoMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeRococoParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeRococoMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49209), added: 51684, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubRococo::LaneToBridge` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubRococo::LaneToBridge` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubRococo::Bridges` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubRococo::Bridges` (`max_values`: None, `max_size`: Some(1918), added: 4393, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn receive_single_message_proof_with_outbound_lane_state() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `701`
		//  Estimated: `52674`
		// Minimum execution time: 65_063_000 picoseconds.
		Weight::from_parts(67_125_000, 0)
			.saturating_add(Weight::from_parts(0, 52674))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgeRococoMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeRococoMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeRococoParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeRococoMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49209), added: 51684, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubRococo::LaneToBridge` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubRococo::LaneToBridge` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubRococo::Bridges` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubRococo::Bridges` (`max_values`: None, `max_size`: Some(1918), added: 4393, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 16384]`.
	fn receive_single_n_bytes_message_proof(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `701`
		//  Estimated: `52674`
		// Minimum execution time: 58_688_000 picoseconds.
		Weight::from_parts(61_404_716, 0)
			.saturating_add(Weight::from_parts(0, 52674))
			// Standard Error: 7
			.saturating_add(Weight::from_parts(2_249, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgeRococoMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeRococoMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeRococoParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoMessages::OutboundLanes` (r:1 w:1)
	/// Proof: `BridgeRococoMessages::OutboundLanes` (`max_values`: None, `max_size`: Some(74), added: 2549, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Storage: `BridgeRelayers::RelayerRewards` (r:1 w:1)
	/// Proof: `BridgeRelayers::RelayerRewards` (`max_values`: None, `max_size`: Some(102), added: 2577, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubRococo::LaneToBridge` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubRococo::LaneToBridge` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubRococo::Bridges` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubRococo::Bridges` (`max_values`: None, `max_size`: Some(1918), added: 4393, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoMessages::OutboundMessages` (r:0 w:1)
	/// Proof: `BridgeRococoMessages::OutboundMessages` (`max_values`: None, `max_size`: Some(65597), added: 68072, mode: `MaxEncodedLen`)
	fn receive_delivery_proof_for_single_message() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `710`
		//  Estimated: `5383`
		// Minimum execution time: 53_123_000 picoseconds.
		Weight::from_parts(54_417_000, 0)
			.saturating_add(Weight::from_parts(0, 5383))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `BridgeRococoMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeRococoMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeRococoParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoMessages::OutboundLanes` (r:1 w:1)
	/// Proof: `BridgeRococoMessages::OutboundLanes` (`max_values`: None, `max_size`: Some(74), added: 2549, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Storage: `BridgeRelayers::RelayerRewards` (r:1 w:1)
	/// Proof: `BridgeRelayers::RelayerRewards` (`max_values`: None, `max_size`: Some(102), added: 2577, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubRococo::LaneToBridge` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubRococo::LaneToBridge` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubRococo::Bridges` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubRococo::Bridges` (`max_values`: None, `max_size`: Some(1918), added: 4393, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoMessages::OutboundMessages` (r:0 w:2)
	/// Proof: `BridgeRococoMessages::OutboundMessages` (`max_values`: None, `max_size`: Some(65597), added: 68072, mode: `MaxEncodedLen`)
	fn receive_delivery_proof_for_two_messages_by_single_relayer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `710`
		//  Estimated: `5383`
		// Minimum execution time: 55_140_000 picoseconds.
		Weight::from_parts(56_456_000, 0)
			.saturating_add(Weight::from_parts(0, 5383))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `BridgeRococoMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeRococoMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeRococoParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoMessages::OutboundLanes` (r:1 w:1)
	/// Proof: `BridgeRococoMessages::OutboundLanes` (`max_values`: None, `max_size`: Some(74), added: 2549, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Storage: `BridgeRelayers::RelayerRewards` (r:2 w:2)
	/// Proof: `BridgeRelayers::RelayerRewards` (`max_values`: None, `max_size`: Some(102), added: 2577, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubRococo::LaneToBridge` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubRococo::LaneToBridge` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubRococo::Bridges` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubRococo::Bridges` (`max_values`: None, `max_size`: Some(1918), added: 4393, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoMessages::OutboundMessages` (r:0 w:2)
	/// Proof: `BridgeRococoMessages::OutboundMessages` (`max_values`: None, `max_size`: Some(65597), added: 68072, mode: `MaxEncodedLen`)
	fn receive_delivery_proof_for_two_messages_by_two_relayers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `710`
		//  Estimated: `6144`
		// Minimum execution time: 60_415_000 picoseconds.
		Weight::from_parts(62_057_000, 0)
			.saturating_add(Weight::from_parts(0, 6144))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `BridgeRococoMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeRococoMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgeRococoParachains::ImportedParaHeads` (`max_values`: Some(64), `max_size`: Some(196), added: 1186, mode: `MaxEncodedLen`)
	/// Storage: `BridgeRococoMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgeRococoMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49209), added: 51684, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubRococo::LaneToBridge` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubRococo::LaneToBridge` (`max_values`: None, `max_size`: Some(65), added: 2540, mode: `MaxEncodedLen`)
	/// Storage: `XcmOverBridgeHubRococo::Bridges` (r:1 w:0)
	/// Proof: `XcmOverBridgeHubRococo::Bridges` (`max_values`: None, `max_size`: Some(1918), added: 4393, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::DeliveryFeeFactor` (r:1 w:0)
	/// Proof: `XcmpQueue::DeliveryFeeFactor` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::RelevantMessagingState` (r:1 w:0)
	/// Proof: `ParachainSystem::RelevantMessagingState` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:1)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: Some(1282), added: 1777, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpMessages` (r:0 w:1)
	/// Proof: `XcmpQueue::OutboundXcmpMessages` (`max_values`: None, `max_size`: Some(105506), added: 107981, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 16384]`.
	fn receive_single_n_bytes_message_proof_with_dispatch(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `965`
		//  Estimated: `52674`
		// Minimum execution time: 84_340_000 picoseconds.
		Weight::from_parts(89_615_003, 0)
			.saturating_add(Weight::from_parts(0, 52674))
			// Standard Error: 15
			.saturating_add(Weight::from_parts(7_574, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
