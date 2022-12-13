use std::sync::Arc;

use crate::{
    components::{nav_bar::NavBar, send_page::SendPage, Component},
    context::ApplicationContext,
};

use super::PageBuilder;

use anyhow::Result;

impl PageBuilder {
    pub async fn send_page(ctx: Arc<ApplicationContext>) -> Result<Box<dyn Component>> {
        Ok(SendPage::new(NavBar::new(ctx.session.clone())))
    }
}
