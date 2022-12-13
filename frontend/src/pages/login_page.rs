use crate::components::account_info::AccountInfoComp;
use crate::components::login_page::LoginPage;
use crate::components::nav_bar::NavBar;
use crate::components::Component;
use crate::context::ApplicationContext;
use crate::pages::PageBuilder;
use anyhow::Result;
use std::sync::Arc;

impl PageBuilder {
    pub async fn login_page(ctx: Arc<ApplicationContext>) -> Result<Box<dyn Component>> {
        let mut account_info_comp: Option<Box<dyn Component>> = None;
        if ctx.session.is_some() {
            account_info_comp = Some(AccountInfoComp::new(
                ctx.session.clone().unwrap().account_info,
            ))
        }
        let nav_bar = NavBar::new(ctx.session.clone());
        let comp = LoginPage::new(nav_bar, account_info_comp);
        Ok(comp)
    }
}
