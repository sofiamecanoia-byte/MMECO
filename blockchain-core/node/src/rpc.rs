//! RPC mínimo para o nó.

use jsonrpsee::RpcModule;

pub fn create_full() -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>> {
    Ok(RpcModule::new(()))
}
