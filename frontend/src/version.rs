use crate::storage::Store;
use anyhow::Result;

const VERSION: u64 = 1;

pub fn reset_on_update(store: Store) -> Result<()> {
    let current_version: u64 = store.get_version();
    if current_version == VERSION {
        return Ok(());
    }

    store.purge()?;
    store.set_version(VERSION)?;
    Ok(())
}
