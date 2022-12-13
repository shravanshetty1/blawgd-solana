use crate::clients::blawgd_client::AccountInfo;
use crate::clients::light_client::light_store::CustomLightStore;
use crate::clients::verification_client::VerificationClient;
use crate::clients::COSMOS_DP;
use anyhow::anyhow;
use anyhow::Result;
use cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::service_client::ServiceClient;
use cosmos_sdk_proto::cosmos::base::tendermint::v1beta1::GetNodeInfoRequest;
use crw_wallet::crypto::MnemonicWallet;
use gloo::storage::errors::StorageError;
use gloo::storage::{LocalStorage, Storage};
use serde::Deserialize;
use serde::Serialize;
use tendermint_light_client::store::LightStore;
use tendermint_light_client::types::{PeerId, Status};

#[derive(Clone)]
pub struct Store;

// TODO refactor this - holy crap

#[derive(Serialize, Deserialize, Clone)]
pub struct ApplicationData {
    pub mnemonic: String,
    pub address: String,
}

const APP_DATA: &str = "app_data";
const VERSION: &str = "version";
const SHOULD_VERIFY: &str = "should_verify";
const PEER_ID: &str = "peer_id";
const LAST_LC_SYNC: &str = "last_lc_sync";
const MAX_NUM_BLOCKS: u64 = 10;

impl Store {
    pub fn set_application_data(&self, app_data: ApplicationData) -> Result<()> {
        Ok(LocalStorage::set(APP_DATA, app_data)?)
    }
    pub fn get_application_data(&self) -> Result<ApplicationData> {
        let app_data: Result<ApplicationData, StorageError> = LocalStorage::get(APP_DATA);
        Ok(app_data?)
    }

    pub fn set_version(&self, version: u64) -> Result<()> {
        Ok(LocalStorage::set(VERSION, version)?)
    }
    pub fn get_version(&self) -> u64 {
        let version: Result<u64, StorageError> = LocalStorage::get(VERSION);
        version.unwrap_or(0)
    }
    pub fn delete_application_data(&self) {
        LocalStorage::delete(APP_DATA)
    }

    pub async fn get_session_account_info(&self, cl: VerificationClient) -> Result<AccountInfo> {
        let address = self.get_application_data()?.address;
        cl.get_account_info(address).await
    }

    pub fn update_last_sync_time(&self) -> Result<()> {
        let unix_ts = chrono::Utc::now().timestamp_millis();
        Ok(LocalStorage::set(LAST_LC_SYNC, unix_ts)?)
    }
    pub fn last_lc_sync(&self) -> Result<i64> {
        let last_lc_sync: Result<i64, StorageError> = LocalStorage::get(LAST_LC_SYNC);
        if last_lc_sync.is_err() {
            return Ok(0);
        }

        Ok(last_lc_sync?)
    }

    pub async fn get_peer_id(&self, grpc: grpc_web_client::Client) -> Result<PeerId> {
        let res: Result<String, StorageError> = LocalStorage::get(PEER_ID);
        let peer_id: String;
        if res.is_err() {
            peer_id = ServiceClient::new(grpc)
                .get_node_info(GetNodeInfoRequest {})
                .await?
                .get_ref()
                .clone()
                .default_node_info
                .ok_or(anyhow!("could not get node info"))?
                .default_node_id;
            LocalStorage::set(PEER_ID, peer_id.clone())?;
        } else {
            peer_id = res?;
        }

        let peer_id = peer_id.parse::<PeerId>()?;
        Ok(peer_id)
    }

    pub fn get_wallet(&self) -> Result<MnemonicWallet> {
        let app_data = self.get_application_data()?;
        Ok(MnemonicWallet::new(app_data.mnemonic.as_str(), COSMOS_DP)?)
    }

    pub fn should_verify(&self) -> Result<bool> {
        let should_verify: Result<bool, StorageError> = LocalStorage::get(SHOULD_VERIFY);
        if should_verify.is_err() {
            return Ok(true);
        }

        self.set_should_verify(true)?;

        Ok(should_verify?)
    }

    pub fn set_should_verify(&self, state: bool) -> Result<()> {
        Ok(LocalStorage::set(SHOULD_VERIFY, state)?)
    }

    pub fn purge(&self) -> Result<()> {
        let local_storage = LocalStorage::raw();
        local_storage
            .clear()
            .map_err(|_| anyhow!("could not clear local storage"))?;
        Ok(())
    }

    pub fn prune_light_store(&self) -> Result<()> {
        let highest = CustomLightStore
            .highest(Status::Trusted)
            .ok_or(anyhow!("could not get highest trusted block"))?
            .signed_header
            .header
            .height
            .value();

        let local_storage = LocalStorage::raw();
        let length = LocalStorage::length();
        for i in 0..length {
            let key: Option<String> = local_storage.key(i).unwrap();
            if key.is_none() {
                continue;
            }

            let key = key.unwrap();
            if !key.starts_with("light-") {
                continue;
            }

            let height: u64 = key
                .strip_prefix("light-")
                .unwrap()
                .split_once("-")
                .unwrap()
                .1
                .parse()
                .unwrap();
            if height < highest - MAX_NUM_BLOCKS {
                LocalStorage::delete(key);
            }
        }

        Ok(())
    }
}
