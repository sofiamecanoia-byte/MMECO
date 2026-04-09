use mmeco_runtime::WASM_BINARY;
use polkadot_sdk::sc_service::ChainType;

/// Especialização da Spec da Chain
pub type ChainSpec = polkadot_sdk::sc_service::GenericChainSpec;

pub fn development_config() -> Result<ChainSpec, String> {
	Ok(ChainSpec::builder(
		WASM_BINARY.ok_or_else(|| "WASM binary não disponível".to_string())?,
		None,
	)
	.with_name("Development")
	.with_id("dev")
	.with_chain_type(ChainType::Development)
	.with_genesis_config_preset_name(polkadot_sdk::sp_genesis_builder::DEV_RUNTIME_PRESET)
	.build())
}

pub fn local_config() -> Result<ChainSpec, String> {
	Ok(ChainSpec::builder(
		WASM_BINARY.ok_or_else(|| "WASM binary não disponível".to_string())?,
		None,
	)
	.with_name("Local Testnet")
	.with_id("local_testnet")
	.with_chain_type(ChainType::Local)
	.with_genesis_config_preset_name(polkadot_sdk::sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET)
	.build())
}
