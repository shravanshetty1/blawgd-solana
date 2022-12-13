use crate::clients::blawgd_client::AccountInfo;
use crate::clients::verification_client::VerificationClient;
use crate::clients::MasterClient;
use crate::dom::Window;
use crate::host::Host;
use crate::logger::Logger;
use crate::storage::Store;
use anyhow::anyhow;
use anyhow::Result;
use cosmos_sdk_proto::cosmos::bank::v1beta1::query_client::QueryClient;
use cosmos_sdk_proto::cosmos::bank::v1beta1::QueryBalanceRequest;

pub struct ApplicationContext {
    pub client: MasterClient,
    pub host: Host,
    pub store: Store,
    pub window: Window,
    pub session: Option<SessionInfo>,
    pub logger: Logger,
}

#[derive(Clone)]
pub struct SessionInfo {
    pub account_info: AccountInfo,
    pub balance: u64,
}

impl SessionInfo {
    pub async fn new(
        store: Store,
        cl: VerificationClient,
        grpc: grpc_web_client::Client,
    ) -> Result<SessionInfo> {
        let address = store.get_application_data()?.address;
        let account_info_resp = cl.get_account_info(address.clone());
        let mut bank_client = QueryClient::new(grpc);
        let balance_resp = bank_client.balance(QueryBalanceRequest {
            address: address.clone(),
            denom: "stake".to_string(),
        });

        let (account_info, balance_resp) =
            futures::future::join(account_info_resp, balance_resp).await;

        let balance = balance_resp?
            .into_inner()
            .balance
            .ok_or(anyhow!("could not get coin from balance resp"))?
            .amount
            .parse::<u64>()?;

        Ok(SessionInfo {
            account_info: account_info?,
            balance,
        })
    }
}
