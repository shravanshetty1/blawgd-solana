use crate::components::account_info::AccountInfoComp;
use crate::components::edit_profile_page::EditProfilePage;
use crate::components::nav_bar::NavBar;
use crate::components::Component;
use crate::context::ApplicationContext;
use crate::pages::PageBuilder;
use anyhow::anyhow;
use anyhow::Result;
use std::sync::Arc;

impl PageBuilder {
    pub async fn edit_profile_page(ctx: Arc<ApplicationContext>) -> Result<Box<dyn Component>> {
        let session = ctx.session.clone();
        let account_info_comp = AccountInfoComp::new(
            session
                .clone()
                .ok_or(anyhow!("user is not logged in"))?
                .account_info,
        );
        let nav_bar = NavBar::new(session.clone());
        let comp = EditProfilePage::new(nav_bar, account_info_comp);
        Ok(comp)
    }
}
