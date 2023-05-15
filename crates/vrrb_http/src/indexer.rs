use http::{HttpClient, HttpClientBuilder};
use mempool::TxnRecord;
use reqwest::StatusCode;
use serde_json;

use crate::{http, Error, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndexerClientConfig {
    pub base_url: String,
}

impl Default for IndexerClientConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:3444".to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IndexerClient {
    client: HttpClient,
}

impl IndexerClient {
    pub fn new(config: IndexerClientConfig) -> Result<Self> {
        let client = HttpClientBuilder::new(config.base_url)?
            .default_headers()
            .build();

        Ok(Self { client })
    }

    pub async fn post_tx(self, txn_record: &TxnRecord) -> Result<StatusCode> {
        let req_json = serde_json::to_string(txn_record).map_err(|e| Error::SerdeJson(e));

        let response = self
            .client
            .post("/transactions", &req_json.unwrap())
            .await?;

        Ok(response.status())
    }
}

#[cfg(test)]
mod tests {
    use mempool::{TxnRecord, TxnStatus};
    use serde_json::json;
    use vrrb_core::txn::{TransactionDigest, Txn};
    use wiremock::{
        http::Method,
        matchers::{method, path},
        Mock,
        MockServer,
        ResponseTemplate,
    };

    use super::*;

    #[tokio::test]
    async fn test_post_tx_success() {
        let mock_server = MockServer::start().await;

        let url = format!("{}{}", "http://", mock_server.address().to_string());

        let config = IndexerClientConfig { base_url: url };
        let client = IndexerClient::new(config).unwrap();

        let txn = Txn::default();
        let txn_record = TxnRecord::new(txn);
        let expected_body = json!(txn_record).to_string();

        let response = ResponseTemplate::new(200).set_body_json(txn_record.to_owned());

        Mock::given(method("POST"))
            .and(path("/transactions"))
            .respond_with(response)
            .mount(&mock_server)
            .await;

        let result = client.post_tx(&txn_record).await;

        assert!(result.is_ok());
        let request_body = mock_server
            .received_requests()
            .await
            .unwrap()
            .pop()
            .unwrap()
            .body
            .to_vec();
    }

    #[tokio::test]
    async fn test_post_tx_failure() {
        let mock_server = wiremock::MockServer::start().await;

        let url = format!("{}{}", "http://", mock_server.address().to_string());

        let mock = wiremock::Mock::given(wiremock::matchers::method("POST"))
            .and(wiremock::matchers::path("/transactions"))
            .respond_with(wiremock::ResponseTemplate::new(500));

        wiremock::Mock::mount(mock, &mock_server).await;

        let base_url = mock_server.address();

        let indexer_config = IndexerClientConfig { base_url: url };
        let indexer_client = IndexerClient::new(indexer_config).unwrap();

        let txn = Txn::default();
        let txn_record = TxnRecord::new(txn);
        let expected_body = json!(txn_record).to_string();

        let result = indexer_client.post_tx(&txn_record).await;

        assert_eq!(result.unwrap(), 500);

        let requests = mock_server.received_requests().await.unwrap();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].method, Method::Post);
        assert_eq!(requests[0].url.path(), "/transactions");
    }
}