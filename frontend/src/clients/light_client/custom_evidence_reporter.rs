use crate::clients::rpc_client::TendermintRPCClient;
use async_trait::async_trait;
use contracts::contract_trait;
use tendermint::abci::transaction::Hash;
use tendermint::evidence::Evidence;
use tendermint_light_client::components::io::IoError;
use tendermint_light_client::evidence::EvidenceReporter;
use tendermint_light_client::types::PeerId;

pub struct CustomEvidenceReporter {
    rpc_client: TendermintRPCClient,
}

impl CustomEvidenceReporter {
    pub fn new(rpc_client: TendermintRPCClient) -> CustomEvidenceReporter {
        CustomEvidenceReporter { rpc_client }
    }
}

#[contract_trait]
#[async_trait(?Send)]
impl EvidenceReporter for CustomEvidenceReporter {
    async fn report(&self, e: Evidence, _: PeerId) -> Result<Hash, IoError> {
        // TODO change this error
        let resp = self
            .rpc_client
            .broadcast_evidence(e)
            .await
            .map_err(|_| IoError::invalid_height())?;
        Ok(resp.hash)
    }
}
