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

//! Autogenerated weights for `pallet_asset_conversion_tx_payment`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-11-07, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-vcatxqpx-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("asset-hub-rococo-dev")`, DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot-parachain
// benchmark
// pallet
// --chain=asset-hub-rococo-dev
// --wasm-execution=compiled
// --pallet=pallet_asset_conversion_tx_payment
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./cumulus/file_header.txt
// --output=./cumulus/parachains/runtimes/assets/asset-hub-rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_asset_conversion_tx_payment`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_asset_conversion_tx_payment::WeightInfo for WeightInfo<T> {
	fn charge_asset_tx_payment_zero() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 733_000 picoseconds.
		Weight::from_parts(802_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: `TransactionPayment::NextFeeMultiplier` (r:1 w:0)
	/// Proof: `TransactionPayment::NextFeeMultiplier` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn charge_asset_tx_payment_native() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `168`
		//  Estimated: `6196`
		// Minimum execution time: 42_169_000 picoseconds.
		Weight::from_parts(43_211_000, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `TransactionPayment::NextFeeMultiplier` (r:1 w:0)
	/// Proof: `TransactionPayment::NextFeeMultiplier` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(808), added: 3283, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Account` (r:2 w:2)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(732), added: 3207, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssetsFreezer::FrozenBalances` (r:2 w:0)
	/// Proof: `ForeignAssetsFreezer::FrozenBalances` (`max_values`: None, `max_size`: Some(682), added: 3157, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn charge_asset_tx_payment_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `614`
		//  Estimated: `7404`
		// Minimum execution time: 123_360_000 picoseconds.
		Weight::from_parts(125_070_000, 0)
			.saturating_add(Weight::from_parts(0, 7404))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}