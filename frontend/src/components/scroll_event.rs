use crate::clients::blawgd_client::PostView;
use crate::clients::verification_client::VerificationClient;
use crate::components::post::PostComponent;
use crate::components::Component;
use crate::context::ApplicationContext;
use crate::task::spawn_local;
use anyhow::anyhow;
use anyhow::Result;
use async_lock::RwLock;
use async_trait::async_trait;
use gloo::events;
use std::sync::Arc;

pub struct PageState {
    pub page: u64,
}

#[derive(Clone)]
pub struct TimelinePostGetter {
    pub address: String,
}

#[async_trait(?Send)]
impl PostGetter for TimelinePostGetter {
    async fn get_post(&self, vc: VerificationClient, page: u64) -> Result<Vec<PostView>> {
        vc.get_timeline_prefetch(self.address.clone(), page).await
    }
}

#[derive(Clone)]
pub struct AccountPostGetter {
    pub address: String,
}

#[async_trait(?Send)]
impl PostGetter for AccountPostGetter {
    async fn get_post(&self, vc: VerificationClient, page: u64) -> Result<Vec<PostView>> {
        vc.get_post_by_account_prefetch(self.address.clone(), page)
            .await
    }
}

#[derive(Clone)]
pub struct ParentPostGetter {
    pub parent_post: String,
}

#[async_trait(?Send)]
impl PostGetter for ParentPostGetter {
    async fn get_post(&self, vc: VerificationClient, page: u64) -> Result<Vec<PostView>> {
        vc.get_post_by_parent_post_prefetch(self.parent_post.clone(), page)
            .await
    }
}

#[async_trait(?Send)]
pub trait PostGetter: Clone {
    async fn get_post(&self, vc: VerificationClient, page: u64) -> Result<Vec<PostView>>;
}

pub fn reg_scroll_event<P: PostGetter + 'static>(
    state: Arc<RwLock<PageState>>,
    ctx: Arc<ApplicationContext>,
    post_getter: P,
) -> Result<()> {
    let window = ctx.window.inner();
    events::EventListener::new(&window, "scroll", move |_| {
        let ctx = ctx.clone();
        let state = state.clone();
        let post_getter = post_getter.clone();
        spawn_local(async move {
            let document = ctx.window.document()?;
            let doc = document.inner().document_element().unwrap();
            let scroll_top: i32 = doc.scroll_top();
            let scroll_height: i32 = doc.scroll_height();
            let client_height: i32 = doc.client_height();

            if scroll_top + client_height >= scroll_height {
                let mut state = state.write().await;
                let posts = post_getter
                    .get_post(ctx.client.vc.clone(), state.page + 1 as u64)
                    .await?;
                if posts.len() == 0 {
                    return Ok(());
                }
                let posts: Vec<Box<PostComponent>> = posts
                    .iter()
                    .map(|p| PostComponent::new(p.clone()))
                    .collect();

                let mut posts_html: String = String::new();
                for post in posts.iter() {
                    posts_html = format!("{}{}", posts_html, post.to_html()?);
                }

                let main_column = document.get_element_by_id("main-column")?.inner();
                main_column
                    .insert_adjacent_html("beforeend", posts_html.as_str())
                    .map_err(|_| anyhow!("could not insert html"))?;

                for post in posts {
                    post.register_events(ctx.clone())?;
                }

                state.page += 1;
                crate::logger::console_log(format!("{}", state.page).as_str());
            }

            Ok(())
        });
    })
    .forget();

    Ok(())
}
