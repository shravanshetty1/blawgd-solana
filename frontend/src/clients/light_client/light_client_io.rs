use crate::clients::rpc_client::TendermintRPCClient;
use anyhow::Result;
use async_trait::async_trait;
use futures::future::try_join;
use io::{AtHeight, Io};
use tendermint::validator::Set;
use tendermint_light_client::{
    components::io,
    components::io::IoError,
    types::{LightBlock, PeerId},
};

pub struct LightClientIO {
    rpc_client: TendermintRPCClient,
    peer_id: PeerId,
}

impl LightClientIO {
    pub fn new(id: PeerId, rpc_client: TendermintRPCClient) -> LightClientIO {
        LightClientIO {
            peer_id: id,
            rpc_client,
        }
    }
}

#[async_trait(?Send)]
impl Io for LightClientIO {
    async fn fetch_light_block(&self, height: AtHeight) -> Result<LightBlock, IoError> {
        // TODO change this
        let res: Result<LightBlock> = async move {
            let height = match height {
                AtHeight::At(height) => height.value(),
                AtHeight::Highest => 0,
            };

            // TODO if height is not zero this can be parallel
            let signed_header = self.rpc_client.get_commit(height).await?.signed_header;
            let height = signed_header.header.height.value();

            let val_resp = self.rpc_client.validators(height);
            let next_val_resp = self.rpc_client.validators(height + 1);
            let (val_resp, next_val_resp) = try_join(val_resp, next_val_resp).await?;

            let validators =
                Set::with_proposer(val_resp.validators, signed_header.header.proposer_address)?;
            let next_validators = Set::without_proposer(next_val_resp.validators);

            Ok(LightBlock {
                signed_header,
                validators,
                next_validators,
                provider: self.peer_id,
            })
        }
        .await;

        res.map_err(|_| IoError::invalid_height())
    }
}
