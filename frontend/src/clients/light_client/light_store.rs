use gloo::storage::errors::StorageError;
use gloo::storage::{LocalStorage, Storage};
use store::LightStore;
use tendermint::block::Height;
use tendermint_light_client::store;
use tendermint_light_client::types::{LightBlock, Status};

#[derive(Debug)]
pub struct CustomLightStore;

// TODO inject storage
impl LightStore for CustomLightStore {
    fn get(&self, height: Height, status: Status) -> Option<LightBlock> {
        LocalStorage::get(light_store_key(status, height)).ok()
    }

    fn update(&mut self, light_block: &LightBlock, status: Status) {
        let height = light_block.signed_header.header.height.clone();
        LocalStorage::set(light_store_key(status, height), light_block).unwrap();
    }

    fn insert(&mut self, light_block: LightBlock, status: Status) {
        let height = light_block.signed_header.header.height.clone();
        LocalStorage::set(light_store_key(status, height), light_block).unwrap();
    }

    fn remove(&mut self, height: Height, status: Status) {
        LocalStorage::delete(light_store_key(status, height));
    }

    fn highest(&self, status: Status) -> Option<LightBlock> {
        let local_storage = LocalStorage::raw();
        let length = LocalStorage::length();

        let mut highest: u64 = u64::MIN;
        for i in 0..length {
            let key: String = local_storage.key(i).unwrap().unwrap();
            if !key.starts_with(format!("light-{}-", status_string(status)).as_str()) {
                continue;
            }

            let height: u64 = key
                .strip_prefix(format!("light-{}-", status_string(status)).as_str())
                .unwrap()
                .parse()
                .unwrap();
            if height > highest {
                highest = height;
            }
        }

        self.get(Height::from(highest as u32), status)
    }

    fn lowest(&self, status: Status) -> Option<LightBlock> {
        let local_storage = LocalStorage::raw();
        let length = LocalStorage::length();

        let mut lowest: u64 = u64::MAX;
        for i in 0..length {
            let key: String = local_storage.key(i).unwrap().unwrap();
            if !key.starts_with(format!("light-{}-", status_string(status)).as_str()) {
                continue;
            }

            let height: u64 = key
                .strip_prefix(format!("light-{}-", status_string(status)).as_str())
                .unwrap()
                .parse()
                .unwrap();
            if height < lowest {
                lowest = height;
            }
        }

        self.get(Height::from(lowest as u32), status)
    }

    fn all(&self, status: Status) -> Box<dyn Iterator<Item = LightBlock>> {
        let local_storage = LocalStorage::raw();
        let length = LocalStorage::length();

        let mut lbs = Vec::new();
        for index in 0..length {
            let key: String = local_storage.key(index).unwrap().unwrap();
            if !key.starts_with(format!("light-{}-", status_string(status)).as_str()) {
                continue;
            }

            let lb: Result<LightBlock, StorageError> = LocalStorage::get(key);
            if lb.is_err() {
                continue;
            }
            lbs.push(lb.unwrap());
        }

        Box::new(lbs.into_iter())
    }
}

fn status_string(s: Status) -> String {
    match s {
        Status::Unverified => "unverified".to_string(),
        Status::Verified => "verified".to_string(),
        Status::Trusted => "trusted".to_string(),
        Status::Failed => "failed".to_string(),
    }
}

fn light_store_key(status: Status, height: Height) -> String {
    format!("light-{}-{}", status_string(status), height)
}
