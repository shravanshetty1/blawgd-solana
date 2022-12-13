use crate::components::nav_bar::NavBar;
use crate::components::post::PostComponent;
use crate::components::post_creator::PostCreator;
use crate::components::timeline_page::TimelinePage;
use crate::components::Component;
use crate::context::ApplicationContext;
use crate::pages::PageBuilder;
use anyhow::anyhow;
use anyhow::Result;
use std::sync::Arc;

impl PageBuilder {
    pub async fn timeline_page(ctx: Arc<ApplicationContext>) -> Result<Box<dyn Component>> {
        let posts = ctx
            .client
            .vc
            .get_timeline_prefetch(
                ctx.session
                    .clone()
                    .ok_or(anyhow!("user not logged in"))?
                    .account_info
                    .address
                    .clone(),
                1,
            )
            .await?;
        let posts = posts
            .iter()
            .map(|p| PostComponent::new(p.clone()) as Box<dyn Component>)
            .collect::<Vec<Box<dyn Component>>>();
        let nav_bar = NavBar::new(ctx.session.clone());
        let mut post_creator: Option<Box<dyn Component>> = None;
        if ctx.session.is_some() {
            post_creator = Some(PostCreator::new("".to_string()));
        }
        let comp = TimelinePage::new(nav_bar, post_creator, posts.into_boxed_slice());
        Ok(comp)
    }
}
