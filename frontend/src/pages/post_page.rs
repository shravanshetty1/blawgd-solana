use crate::components::nav_bar::NavBar;
use crate::components::post::PostComponent;
use crate::components::post_creator::PostCreator;
use crate::components::post_page::PostPage;
use crate::components::Component;
use crate::context::ApplicationContext;
use crate::pages::PageBuilder;
use anyhow::anyhow;
use anyhow::Result;
use std::sync::Arc;

impl PageBuilder {
    pub async fn post_page(ctx: Arc<ApplicationContext>) -> Result<Box<dyn Component>> {
        let url: String = ctx.window.location().href()?;
        let post_id = url
            .as_str()
            .strip_prefix(format!("{}/post/", ctx.host.endpoint()).as_str())
            .ok_or(anyhow!("could not get post id from {}", url.clone()))?
            .to_string();

        let posts = ctx
            .client
            .vc
            .get_post_by_parent_post_prefetch(post_id.clone(), 1);
        let main_post = ctx.client.vc.get_post(post_id.clone());
        let (posts, main_post) = futures::future::try_join(posts, main_post).await?;

        let mut main_post = PostComponent::new(main_post);
        main_post.as_mut().focus();
        let nav_bar = NavBar::new(ctx.session.clone());
        let mut post_creator_component: Option<Box<dyn Component>> = None;
        if ctx.session.is_some() {
            let mut post_creator = PostCreator::new(post_id);
            post_creator.as_mut().set_button_text("Reply");
            post_creator_component = Some(post_creator);
        }
        let mut boxed_posts: Vec<Box<dyn Component>> = Vec::new();
        for post in posts {
            boxed_posts.push(PostComponent::new(post))
        }
        let comp = PostPage::new(
            nav_bar,
            main_post,
            post_creator_component,
            boxed_posts.into_boxed_slice(),
        );

        Ok(comp)
    }
}
