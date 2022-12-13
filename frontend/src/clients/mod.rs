use crate::clients::rpc_client::TendermintRPCClient;
use crate::clients::verification_client::VerificationClient;
pub mod blawgd_client;
pub mod cosmos_client;
pub mod light_client;
pub mod rpc_client;
pub mod verification_client;

use crate::clients::cosmos_client::CosmosClient;
use crate::clients::light_client::LightClient;

pub const COSMOS_DP: &str = "m/44'/118'/0'/0/0";
pub const ADDRESS_HRP: &str = "blawgd";

pub struct MasterClient {
    pub lc: LightClient,
    pub vc: VerificationClient,
    pub rpc: TendermintRPCClient,
    pub cosmos: CosmosClient<grpc_web_client::Client>,
}
