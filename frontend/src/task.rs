use anyhow::Result;
use std::future::Future;

pub fn spawn_local<I>(task: I)
where
    I: Future<Output = Result<()>> + 'static,
{
    wasm_bindgen_futures::spawn_local(async move {
        let resp = task.await;
        if resp.is_err() {
            crate::logger::console_log(resp.err().unwrap().to_string().as_str());
        }
    })
}
