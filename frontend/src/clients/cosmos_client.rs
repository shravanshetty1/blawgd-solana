use crate::clients::ADDRESS_HRP;
use anyhow::anyhow;
use anyhow::Result;
use cosmos_sdk_proto::cosmos::auth::v1beta1::query_client::QueryClient;
use cosmos_sdk_proto::cosmos::auth::v1beta1::{BaseAccount, QueryAccountRequest};
use cosmos_sdk_proto::cosmos::tx::v1beta1::service_client::ServiceClient;
use cosmos_sdk_proto::cosmos::tx::v1beta1::{
    BroadcastMode, BroadcastTxRequest, BroadcastTxResponse, Tx, TxRaw,
};
use crw_client::tx::TxBuilder;
use crw_wallet::crypto::MnemonicWallet;
use tonic::codegen::Body;
use tonic::codegen::StdError;
use tonic::Response;

pub struct CosmosClient<T>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody>,
    T::ResponseBody: Body + Send + 'static,
    T::Error: Into<StdError>,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    T: Clone,
{
    pub client: T,
}

pub const MSG_BANK_SEND: &str = "/cosmos.bank.v1beta1.MsgSend";
pub const CHAIN_ID: &str = "blawgd";
pub const MEMO: &str = "browser client";

impl<T> CosmosClient<T>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody>,
    T::ResponseBody: Body + Send + 'static,
    T::Error: Into<StdError>,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    T: Clone,
{
    pub async fn broadcast_tx<M: prost::Message>(
        &self,
        wallet: MnemonicWallet,
        msg_type: &str,
        msg: M,
    ) -> Result<Response<BroadcastTxResponse>> {
        let acc_resp = QueryClient::new(self.client.clone())
            .account(QueryAccountRequest {
                address: wallet.get_bech32_address(ADDRESS_HRP)?,
            })
            .await?;

        let account_data: BaseAccount =
            prost::Message::decode(acc_resp.get_ref().account.as_ref().unwrap().value.as_ref())?;
        let tx = TxBuilder::new(CHAIN_ID)
            .memo(MEMO)
            .account_info(account_data.sequence, account_data.account_number)
            .timeout_height(0)
            .fee("stake", "0", 3000000)
            .add_message(msg_type, msg)?
            .sign(&wallet)?;
        let tx_raw = serialize_tx(&tx)?;

        let resp = ServiceClient::new(self.client.clone())
            .broadcast_tx(BroadcastTxRequest {
                tx_bytes: tx_raw,
                mode: BroadcastMode::Block as i32,
            })
            .await?;

        let tx_resp = resp
            .get_ref()
            .tx_response
            .clone()
            .ok_or(anyhow!("could not get tx response"))?;
        let status = tx_resp.code.clone();
        if status != 0 {
            return Err(anyhow!("transaction failed - {}", tx_resp.raw_log));
        }

        Ok(resp)
    }
}

pub fn serialize_tx(tx: &Tx) -> Result<Vec<u8>> {
    let mut serialized_body: Vec<u8> = Vec::new();
    let mut serialized_auth: Vec<u8> = Vec::new();
    let mut serialized_tx: Vec<u8> = Vec::new();

    // Serialize the tx body and auth_info
    if let Some(body) = &tx.body {
        prost::Message::encode(body, &mut serialized_body)?;
    }
    if let Some(auth_info) = &tx.auth_info {
        prost::Message::encode(auth_info, &mut serialized_auth)?;
    }

    // Prepare and serialize the TxRaw
    let tx_raw = TxRaw {
        body_bytes: serialized_body,
        auth_info_bytes: serialized_auth,
        signatures: tx.signatures.clone(),
    };
    prost::Message::encode(&tx_raw, &mut serialized_tx)?;
    Ok(serialized_tx)
}
