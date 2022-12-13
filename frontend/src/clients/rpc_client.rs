use crate::host::Host;
use anyhow::Result;
use tendermint::evidence::Evidence;
use tendermint_rpc::endpoint::{block, commit, evidence, validators};
use tendermint_rpc::response::Wrapper;

#[derive(Clone)]
pub struct TendermintRPCClient {
    client: reqwest::Client,
    host: Host,
}

impl TendermintRPCClient {
    pub fn new(host: Host) -> Result<TendermintRPCClient> {
        Ok(TendermintRPCClient {
            host,
            client: reqwest::Client::builder().build()?,
        })
    }

    pub async fn get_block(&self, height: u64) -> Result<block::Response> {
        let mut param: String = String::new();
        if height != 0 {
            param = format!("?height={}", height)
        }

        let resp = self
            .client
            .get(format!("{}/block{}", self.host.tendermint_endpoint(), param).as_str())
            .send()
            .await?
            .json::<Wrapper<block::Response>>()
            .await?
            .into_result()?;

        Ok(resp)
    }

    pub async fn get_commit(&self, height: u64) -> Result<commit::Response> {
        let mut param: String = String::new();
        if height != 0 {
            param = format!("?height={}", height)
        }

        let resp = self
            .client
            .get(format!("{}/commit{}", self.host.tendermint_endpoint(), param).as_str())
            .send()
            .await?
            .json::<Wrapper<commit::Response>>()
            .await?
            .into_result()?;

        Ok(resp)
    }

    pub async fn broadcast_evidence(&self, e: Evidence) -> Result<evidence::Response> {
        let evidence = serde_json::to_string(&e)?;

        let resp = self
            .client
            .get(
                format!(
                    "{}/broadcast_evidence?evidence={}",
                    self.host.tendermint_endpoint(),
                    evidence
                )
                .as_str(),
            )
            .send()
            .await?
            .json::<Wrapper<evidence::Response>>()
            .await?
            .into_result()?;

        Ok(resp)
    }

    pub async fn validators(&self, height: u64) -> Result<validators::Response> {
        let mut param: String = String::new();
        if height != 0 {
            param = format!("?height={}", height)
        }

        let resp = self
            .client
            .get(format!("{}/validators{}", self.host.tendermint_endpoint(), param).as_str())
            .send()
            .await?
            .json::<Wrapper<validators::Response>>()
            .await?
            .into_result()?;

        Ok(resp)
    }
}
