use std::sync::Arc;

use crate::{clients::blawgd_client::MSG_BANK_SEND, ApplicationContext};
use crate::{components::Component, task::spawn_local};
use anyhow::anyhow;
use anyhow::Result;
use cosmos_sdk_proto::cosmos::{bank::v1beta1::MsgSend, base::v1beta1::Coin};
use gloo::events;
use wasm_bindgen::JsCast;

const DENOM: &str = "stake";

pub struct SendPage {
    nav_bar: Box<dyn Component>,
}

impl SendPage {
    pub fn new(nav_bar: Box<dyn Component>) -> Box<SendPage> {
        Box::new(SendPage { nav_bar })
    }
}

impl super::Component for SendPage {
    fn to_html(&self) -> Result<String> {
        let html = String::from(format!(
            r#"
<div class="page">
    <div class="page-wrapper">
        {}
        <div id="main-column" class="main-column">
        <div class="send-component">
                <div class="login-notice">Please fill the given fields to send coins to someone</div>
                <div class="send-component-inner-wrapper">
                    <div class="new-account-info">
                        <input id="address-field" class="account-info-field" type="text" placeholder="Address to send to...">
                        <input id="amount-field" class="account-info-field" type="text" placeholder="Amount to send...">
                    </div>
                    <div id="send" class="button">Send</div>
                </div>
            </div>
        </div>
    </div>
</div>
"#,
            self.nav_bar.to_html()?,
        ));

        Ok(html)
    }

    fn register_events(&self, ctx: Arc<ApplicationContext>) -> Result<()> {
        self.nav_bar.register_events(ctx.clone())?;

        let document = ctx.window.document()?;
        let send_button = document.get_element_by_id("send")?.inner();
        events::EventListener::new(&send_button, "click", move |_| {
            let ctx = ctx.clone();
            let document = document.clone();
            spawn_local(async move {
                let address: String = document
                    .get_element_by_id("address-field")?
                    .inner()
                    .dyn_ref::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value();
                let amount: String = document
                    .get_element_by_id("amount-field")?
                    .inner()
                    .dyn_ref::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value()
                    .parse::<u64>()?
                    .to_string();

                let msg = MsgSend {
                    from_address: ctx
                        .session
                        .as_ref()
                        .ok_or(anyhow!("user not logged in"))?
                        .account_info
                        .address
                        .clone(),
                    to_address: address,
                    amount: vec![Coin {
                        denom: DENOM.to_string(),
                        amount: amount.clone(),
                    }],
                };
                ctx.client
                    .cosmos
                    .broadcast_tx(ctx.store.get_wallet()?, MSG_BANK_SEND, msg)
                    .await?;

                ctx.window
                    .location()
                    .inner()
                    .reload()
                    .map_err(|_| anyhow!("could not reload page"))?;

                Ok(())
            });
        })
        .forget();

        Ok(())
    }
}
