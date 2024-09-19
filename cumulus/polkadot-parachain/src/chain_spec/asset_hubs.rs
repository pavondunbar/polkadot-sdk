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

use crate::chain_spec::{get_account_id_from_seed, get_collator_keys_from_seed, SAFE_XCM_VERSION};
use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use parachains_common::{AccountId, AuraId, Balance as AssetHubBalance};
use polkadot_parachain_lib::chain_spec::{Extensions, GenericChainSpec};
use sc_service::ChainType;
use sp_core::{crypto::UncheckedInto, sr25519};

const ASSET_HUB_WESTEND_ED: AssetHubBalance = asset_hub_westend_runtime::ExistentialDeposit::get();

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn asset_hub_westend_session_keys(keys: AuraId) -> asset_hub_westend_runtime::SessionKeys {
	asset_hub_westend_runtime::SessionKeys { aura: keys }
}

pub fn asset_hub_westend_development_config() -> GenericChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "WND".into());
	properties.insert("tokenDecimals".into(), 12.into());

	GenericChainSpec::builder(
		asset_hub_westend_runtime::WASM_BINARY
			.expect("WASM binary was not built, please build it!"),
		Extensions { relay_chain: "westend".into(), para_id: 1000 },
	)
	.with_name("Westend Asset Hub Development")
	.with_id("asset-hub-westend-dev")
	.with_chain_type(ChainType::Local)
	.with_genesis_config_patch(asset_hub_westend_genesis(
		// initial collators.
		vec![(
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_collator_keys_from_seed::<AuraId>("Alice"),
		)],
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
		],
		testnet_parachains_constants::westend::currency::UNITS * 1_000_000,
		1000.into(),
	))
	.with_properties(properties)
	.build()
}

pub fn asset_hub_westend_local_config() -> GenericChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "WND".into());
	properties.insert("tokenDecimals".into(), 12.into());

	GenericChainSpec::builder(
		asset_hub_westend_runtime::WASM_BINARY
			.expect("WASM binary was not built, please build it!"),
		Extensions { relay_chain: "westend-local".into(), para_id: 1000 },
	)
	.with_name("Westend Asset Hub Local")
	.with_id("asset-hub-westend-local")
	.with_chain_type(ChainType::Local)
	.with_genesis_config_patch(asset_hub_westend_genesis(
		// initial collators.
		vec![
			(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_collator_keys_from_seed::<AuraId>("Alice"),
			),
			(
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_collator_keys_from_seed::<AuraId>("Bob"),
			),
		],
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
		testnet_parachains_constants::westend::currency::UNITS * 1_000_000,
		1000.into(),
	))
	.with_properties(properties)
	.build()
}

pub fn asset_hub_westend_config() -> GenericChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "WND".into());
	properties.insert("tokenDecimals".into(), 12.into());

	GenericChainSpec::builder(
		asset_hub_westend_runtime::WASM_BINARY
			.expect("WASM binary was not built, please build it!"),
		Extensions { relay_chain: "westend".into(), para_id: 1000 },
	)
	.with_name("Westend Asset Hub")
	.with_id("asset-hub-westend")
	.with_chain_type(ChainType::Live)
	.with_genesis_config_patch(asset_hub_westend_genesis(
		// initial collators.
		vec![
			(
				hex!("9cfd429fa002114f33c1d3e211501d62830c9868228eb3b4b8ae15a83de04325").into(),
				hex!("9cfd429fa002114f33c1d3e211501d62830c9868228eb3b4b8ae15a83de04325")
					.unchecked_into(),
			),
			(
				hex!("12a03fb4e7bda6c9a07ec0a11d03c24746943e054ff0bb04938970104c783876").into(),
				hex!("12a03fb4e7bda6c9a07ec0a11d03c24746943e054ff0bb04938970104c783876")
					.unchecked_into(),
			),
			(
				hex!("1256436307dfde969324e95b8c62cb9101f520a39435e6af0f7ac07b34e1931f").into(),
				hex!("1256436307dfde969324e95b8c62cb9101f520a39435e6af0f7ac07b34e1931f")
					.unchecked_into(),
			),
			(
				hex!("98102b7bca3f070f9aa19f58feed2c0a4e107d203396028ec17a47e1ed80e322").into(),
				hex!("98102b7bca3f070f9aa19f58feed2c0a4e107d203396028ec17a47e1ed80e322")
					.unchecked_into(),
			),
		],
		Vec::new(),
		ASSET_HUB_WESTEND_ED * 4096,
		1000.into(),
	))
	.with_properties(properties)
	.build()
}

fn asset_hub_westend_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	endowment: AssetHubBalance,
	id: ParaId,
) -> serde_json::Value {
	serde_json::json!({
		"balances": {
			"balances": endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, endowment))
				.collect::<Vec<_>>(),
		},
		"parachainInfo": {
			"parachainId": id,
		},
		"collatorSelection": {
			"invulnerables": invulnerables.iter().cloned().map(|(acc, _)| acc).collect::<Vec<_>>(),
			"candidacyBond": ASSET_HUB_WESTEND_ED * 16,
		},
		"session": {
			"keys": invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                          // account id
						acc,                                  // validator id
						asset_hub_westend_session_keys(aura), // session keys
					)
				})
				.collect::<Vec<_>>(),
		},
		"polkadotXcm": {
			"safeXcmVersion": Some(SAFE_XCM_VERSION),
		},
	})
}

pub fn asset_hub_rococo_development_config() -> GenericChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("ss58Format".into(), 42.into());
	properties.insert("tokenSymbol".into(), "ROC".into());
	properties.insert("tokenDecimals".into(), 12.into());
	asset_hub_rococo_like_development_config(
		properties,
		"Rococo Asset Hub Development",
		"asset-hub-rococo-dev",
		1000,
	)
}

fn asset_hub_rococo_like_development_config(
	properties: sc_chain_spec::Properties,
	name: &str,
	chain_id: &str,
	para_id: u32,
) -> GenericChainSpec {
	GenericChainSpec::builder(
		asset_hub_rococo_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
		Extensions { relay_chain: "rococo-dev".into(), para_id },
	)
	.with_name(name)
	.with_id(chain_id)
	.with_chain_type(ChainType::Local)
	.with_genesis_config_preset_name("development")
	.with_properties(properties)
	.build()
}

pub fn asset_hub_rococo_local_config() -> GenericChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("ss58Format".into(), 42.into());
	properties.insert("tokenSymbol".into(), "ROC".into());
	properties.insert("tokenDecimals".into(), 12.into());
	asset_hub_rococo_like_local_config(
		properties,
		"Rococo Asset Hub Local",
		"asset-hub-rococo-local",
		1000,
	)
}

fn asset_hub_rococo_like_local_config(
	properties: sc_chain_spec::Properties,
	name: &str,
	chain_id: &str,
	para_id: u32,
) -> GenericChainSpec {
	GenericChainSpec::builder(
		asset_hub_rococo_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
		Extensions { relay_chain: "rococo-local".into(), para_id },
	)
	.with_name(name)
	.with_id(chain_id)
	.with_chain_type(ChainType::Local)
	.with_genesis_config_preset_name("local")
	.with_properties(properties)
	.build()
}

pub fn asset_hub_rococo_genesis_config() -> GenericChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "ROC".into());
	properties.insert("tokenDecimals".into(), 12.into());
	let para_id = 1000;
	GenericChainSpec::builder(
		asset_hub_rococo_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
		Extensions { relay_chain: "rococo".into(), para_id },
	)
	.with_name("Rococo Asset Hub")
	.with_id("asset-hub-rococo")
	.with_chain_type(ChainType::Live)
	.with_genesis_config_preset_name("genesis")
	.with_properties(properties)
	.build()
}
