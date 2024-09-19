// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.
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

//! ChainSpecs dedicated to Rococo parachain setups (for testing and example purposes)

use crate::chain_spec::{get_from_seed, SAFE_XCM_VERSION};
use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use parachains_common::AccountId;
use polkadot_parachain_lib::chain_spec::{Extensions, GenericChainSpec};
use polkadot_service::chain_spec::get_account_id_from_seed;
use rococo_parachain_runtime::AuraId;
use sc_chain_spec::ChainType;
use sp_core::{crypto::UncheckedInto, sr25519};

pub fn rococo_parachain_local_config() -> GenericChainSpec {
	GenericChainSpec::builder(
		rococo_parachain_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
		Extensions { relay_chain: "rococo-local".into(), para_id: 1000 },
	)
	.with_name("Rococo Parachain Local")
	.with_id("local_testnet")
	.with_chain_type(ChainType::Local)
	.with_genesis_config_patch(testnet_genesis(
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		vec![get_from_seed::<AuraId>("Alice"), get_from_seed::<AuraId>("Bob")],
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
			get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
			get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		],
		1000.into(),
	))
	.build()
}

pub fn staging_rococo_parachain_local_config() -> GenericChainSpec {
	#[allow(deprecated)]
	GenericChainSpec::builder(
		rococo_parachain_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
		Extensions { relay_chain: "rococo-local".into(), para_id: 1000 },
	)
	.with_name("Staging Rococo Parachain Local")
	.with_id("staging_testnet")
	.with_chain_type(ChainType::Live)
	.with_genesis_config_patch(testnet_genesis(
		hex!["9ed7705e3c7da027ba0583a22a3212042f7e715d3c168ba14f1424e2bc111d00"].into(),
		vec![
			// $secret//one
			hex!["aad9fa2249f87a210a0f93400b7f90e47b810c6d65caa0ca3f5af982904c2a33"]
				.unchecked_into(),
			// $secret//two
			hex!["d47753f0cca9dd8da00c70e82ec4fc5501a69c49a5952a643d18802837c88212"]
				.unchecked_into(),
		],
		vec![hex!["9ed7705e3c7da027ba0583a22a3212042f7e715d3c168ba14f1424e2bc111d00"].into()],
		1000.into(),
	))
	.build()
}

pub(crate) fn testnet_genesis(
	root_key: AccountId,
	initial_authorities: Vec<AuraId>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> serde_json::Value {
	serde_json::json!({
		"balances": {
			"balances": endowed_accounts.iter().cloned().map(|k| (k, 1u64 << 60)).collect::<Vec<_>>(),
		},
		"sudo": { "key": Some(root_key) },
		"parachainInfo": {
			"parachainId": id,
		},
		"aura": { "authorities": initial_authorities },
		"polkadotXcm": {
			"safeXcmVersion": Some(SAFE_XCM_VERSION),
		},
	})
}
