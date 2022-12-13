use crate::components::faucet_page::FaucetPage;
use crate::components::nav_bar::NavBar;
use crate::components::Component;
use crate::context::ApplicationContext;
use crate::pages::PageBuilder;
use anyhow::Result;
use std::sync::Arc;

impl PageBuilder {
    pub async fn faucet_page(ctx: Arc<ApplicationContext>) -> Result<Box<dyn Component>> {
        let session = ctx.session.clone();
        let nav_bar = NavBar::new(session);
        let site_key =
            reqwest::get(format!("{}{}", ctx.host.faucet_endpoint(), "/sitekey").as_str())
                .await?
                .text()
                .await?;
        let page = FaucetPage::new(nav_bar, site_key);
        Ok(page)
    }
}
