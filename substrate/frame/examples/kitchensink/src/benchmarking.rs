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

//! Benchmarking for `pallet-example-kitchensink`.

// Only enable this module for benchmarking.
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Kitchensink;

use frame_benchmarking::v2::*;
use frame_support::pallet_prelude::TransactionSource;
use frame_system::RawOrigin;

// To actually run this benchmark on pallet-example-kitchensink, we need to put this pallet into the
//   runtime and compile it with `runtime-benchmarks` feature. The detail procedures are
//   documented at:
//   https://docs.substrate.io/reference/how-to-guides/weights/add-benchmarks/
//
// The auto-generated weight estimate of this pallet is copied over to the `weights.rs` file.
// The exact command of how the estimate generated is printed at the top of the file.

// Details on using the benchmarks macro can be seen at:
//   https://paritytech.github.io/substrate/master/frame_benchmarking/trait.Benchmarking.html#tymethod.benchmarks
#[benchmarks]
mod benchmarks {
	use super::*;

	// This will measure the execution time of `set_foo`.
	#[benchmark]
	fn set_foo_benchmark() {
		// This is the benchmark setup phase.
		// `set_foo` is a constant time function, hence we hard-code some random value here.
		let value = 1000u32.into();
		#[extrinsic_call]
		set_foo(RawOrigin::Root, value, 10u128); // The execution phase is just running `set_foo` extrinsic call

		// This is the optional benchmark verification phase, asserting certain states.
		assert_eq!(Foo::<T>::get(), Some(value))
	}

	// This will measure the execution time of `set_foo_using_authorize`.
	#[benchmark]
	fn set_foo_using_authorize() {
		// This is the benchmark setup phase.

		// `set_foo_using_authorize` is only authorized when value is 42 so we will use it.
		let value = 42u32;
		// We dispatch with authorized origin, it is the origin resulting from authorization.
		let origin = RawOrigin::Authorized;

		#[extrinsic_call]
		_(origin, value); // The execution phase is just running `set_foo_using_authorize` extrinsic call

		// This is the optional benchmark verification phase, asserting certain states.
		assert_eq!(Foo::<T>::get(), Some(42))
	}

	// This will measure the weight for the closure in `[pallet::authorize(...)]`.
	#[benchmark]
	fn authorize_set_foo_using_authorize() {
		// This is the benchmark setup phase.

		let call = Call::<T>::set_foo_using_authorize { new_foo: 42 };
		let source = TransactionSource::External;

		// We use a block with specific code to benchmark the closure.
		#[block]
		{
			use frame_support::traits::Authorize;
			call.authorize(source)
				.expect("Call give some authorization")
				.expect("Authorization is successful");
		}
	}

	// This line generates test cases for benchmarking, and could be run by:
	//   `cargo test -p pallet-example-kitchensink --all-features`, you will see one line per case:
	//   `test benchmarking::bench_set_foo_benchmark ... ok`
	//   `test benchmarking::bench_set_foo_using_authorize_benchmark ... ok` in the result.
	//   `test benchmarking::bench_authorize_set_foo_using_authorize_benchmark ... ok` in the result.
	//
	// The line generates three steps per benchmark, with repeat=1 and the three steps are
	//   [low, mid, high] of the range.
	impl_benchmark_test_suite!(Kitchensink, crate::tests::new_test_ext(), crate::tests::Test);
}
