use std::net::SocketAddr;

use crate::ApiError;
use jsonrpsee::core::client::Client;
use jsonrpsee::ws_client::WsClientBuilder;

pub async fn create_client(server_url: SocketAddr) -> crate::Result<Client> {
    let jsonrpc_url = format!("ws://{}", server_url);

    let client = WsClientBuilder::default()
        .build(&jsonrpc_url)
        .await
        .map_err(|err| ApiError::Other(format!("unable to satrt JSON-RPC server: {}", err)))?;

    Ok(client)
}