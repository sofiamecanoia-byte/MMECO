use crate::{AccountId, RuntimeGenesisConfig};
use alloc::{vec, vec::Vec};
use polkadot_sdk::sp_consensus_aura::sr25519::AuthorityId as AuraId;
use polkadot_sdk::sp_consensus_grandpa::AuthorityId as GrandpaId;
use polkadot_sdk::sp_genesis_builder::PresetId;
use polkadot_sdk::sp_keyring::AccountKeyring;
use serde_json::Value;

fn testnet_genesis(
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	endowed_accounts: Vec<AccountId>,
) -> Value {
	let config = RuntimeGenesisConfig {
		balances: polkadot_sdk::pallet_balances::GenesisConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 1u128 << 60))
				.collect::<Vec<_>>(),
		},
		aura: polkadot_sdk::pallet_aura::GenesisConfig {
			authorities: initial_authorities.iter().map(|x| x.0.clone()).collect::<Vec<_>>(),
		},
		grandpa: polkadot_sdk::pallet_grandpa::GenesisConfig {
			authorities: initial_authorities
				.iter()
				.map(|x| (x.1.clone(), 1))
				.collect::<Vec<_>>(),
			..Default::default()
		},
		..Default::default()
	};

	serde_json::to_value(config).expect("Could not build genesis config.")
}

pub fn development_config_genesis() -> Value {
	testnet_genesis(
		vec![(
			polkadot_sdk::sp_keyring::Sr25519Keyring::Alice.public().into(),
			polkadot_sdk::sp_keyring::Ed25519Keyring::Alice.public().into(),
		)],
		vec![
			AccountKeyring::Alice.to_account_id(),
			AccountKeyring::Bob.to_account_id(),
		],
	)
}

pub fn local_config_genesis() -> Value {
	testnet_genesis(
		vec![
			(
				polkadot_sdk::sp_keyring::Sr25519Keyring::Alice.public().into(),
				polkadot_sdk::sp_keyring::Ed25519Keyring::Alice.public().into(),
			),
			(
				polkadot_sdk::sp_keyring::Sr25519Keyring::Bob.public().into(),
				polkadot_sdk::sp_keyring::Ed25519Keyring::Bob.public().into(),
			),
		],
		AccountKeyring::iter().map(|v| v.to_account_id()).collect::<Vec<_>>(),
	)
}

pub fn get_preset(id: &PresetId) -> Option<Vec<u8>> {
	let patch = match id.as_ref() {
		polkadot_sdk::sp_genesis_builder::DEV_RUNTIME_PRESET => development_config_genesis(),
		polkadot_sdk::sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET => local_config_genesis(),
		_ => return None,
	};
	Some(
		serde_json::to_string(&patch)
			.expect("serialization to json is expected to work. qed.")
			.into_bytes(),
	)
}

pub fn preset_names() -> Vec<PresetId> {
	vec![
		PresetId::from(polkadot_sdk::sp_genesis_builder::DEV_RUNTIME_PRESET),
		PresetId::from(polkadot_sdk::sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET),
	]
}
