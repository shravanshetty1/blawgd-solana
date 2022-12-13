use crate::clients::blawgd_client::{
    MsgLikePost, MsgRepost, PostView, MSG_TYPE_LIKE, MSG_TYPE_REPOST,
};
use crate::context::ApplicationContext;
use crate::task::spawn_local;
use anyhow::anyhow;
use anyhow::Result;
use gloo::events::EventListener;
use std::sync::Arc;

pub struct PostComponent {
    pub post: PostView,
    focus: bool,
}

impl PostComponent {
    pub fn new(post: PostView) -> Box<PostComponent> {
        Box::new(PostComponent { post, focus: false })
    }
    pub fn focus(&mut self) {
        self.focus = true;
    }

    fn like_event(&self, ctx: Arc<ApplicationContext>) -> Result<()> {
        let post = self.post.clone();
        let document = ctx.window.document()?;
        let like_button_wrapper_id = format!("post-{}-like", post.id);
        let like_button_wrapper = document.get_element_by_id(like_button_wrapper_id.as_str())?;
        EventListener::new(&like_button_wrapper.inner(), "click", move |_| {
            let post = post.clone();
            let ctx = ctx.clone();
            let document = document.clone();

            // TODO rerender the entire component

            spawn_local(async move {
                let session = ctx
                    .session
                    .as_ref()
                    .ok_or(anyhow!("could not get session account info"))?;

                let _resp = ctx
                    .client
                    .cosmos
                    .broadcast_tx(
                        ctx.store.get_wallet()?,
                        MSG_TYPE_LIKE,
                        MsgLikePost {
                            creator: session.account_info.address.clone(),
                            post_id: post.id.clone(),
                            amount: 1,
                        },
                    )
                    .await?;

                let like_button_id = format!("post-{}-like-content", post.id);
                let like_button = document.get_element_by_id(like_button_id.as_str())?;
                let like_button_text = like_button.inner_html();
                let mut likes_count = like_button_text.parse::<i32>().unwrap_or(0);
                likes_count += 1;
                like_button.set_inner_html(format!("{}", likes_count).as_str());

                Ok(())
            });
        })
        .forget();
        Ok(())
    }

    fn repost_event(&self, ctx: Arc<ApplicationContext>) -> Result<()> {
        let post = self.post.clone();
        let document = ctx.window.document()?;
        let repost_button_wrapper_id = format!("post-{}-repost", post.id);
        let repost_button_wrapper =
            document.get_element_by_id(repost_button_wrapper_id.as_str())?;
        EventListener::new(&repost_button_wrapper.inner(), "click", move |_| {
            let post = post.clone();
            let ctx = ctx.clone();
            let document = document.clone();
            spawn_local(async move {
                let session = ctx
                    .session
                    .as_ref()
                    .ok_or(anyhow!("could not get session account info"))?;

                let _resp = ctx
                    .client
                    .cosmos
                    .broadcast_tx(
                        ctx.store.get_wallet()?,
                        MSG_TYPE_REPOST,
                        MsgRepost {
                            creator: session.account_info.address.clone(),
                            post_id: post.id.clone(),
                        },
                    )
                    .await?;

                let repost_button_id = format!("post-{}-repost-content", post.id);
                let repost_button = document.get_element_by_id(repost_button_id.as_str())?;
                let repost_button_text: String = repost_button.inner_html();
                let mut repost_count = repost_button_text.parse::<i32>().unwrap_or(0);
                repost_count += 1;
                repost_button.set_inner_html(format!("{}", repost_count).as_str());

                Ok(())
            });
        })
        .forget();
        Ok(())
    }
}

impl super::Component for PostComponent {
    fn to_html(&self) -> Result<String> {
        let mut account_info = self.post.creator.clone().unwrap();

        let mut post_text_class = "post-component-text";
        if self.focus {
            post_text_class = "post-component-text-focus";
        }

        let parent_post = self.post.parent_post.clone();
        let mut post_header: String = String::new();
        if !parent_post.is_empty() {
            post_header = format!(
                r#"<a href="/post/{}" class="post-component-header">Replying to post {}</a>"#,
                parent_post, parent_post
            )
            .to_string();
        }

        let mut post = self.post.clone();
        if post.repost_parent.is_some() {
            let old_post_header = post_header.clone();
            post_header = format!(
                r#"<a href="/profile/{}" class="post-component-header">Reposted by {}</a>"#,
                account_info.address, account_info.name
            )
            .to_string();
            post_header.push_str(old_post_header.as_str());
            let repost = post.repost_parent.unwrap().as_ref().clone();
            post.content = repost.content.clone();
            account_info = repost.creator.clone().unwrap();
        }

        let html = String::from(format!(
            r#"
            <div class="post-component">
                {}
                <div class="post-component-text-wrapper">
                    <a href="/profile/{}"><img src="{}" class="post-component-account-info-image"></a>
                    <div class="post-component-text-content">
                        <div class="post-component-account-info">
                            <a href="/profile/{}" class="post-component-account-info-name">{}</a>
                            <div class="post-component-account-info-address">@{}</div>
                        </div>
                        <div class="{}">
                            {}
                        </div>
                    </div>
                </div>
                <div class="post-component-bar">
                    <div id="post-{}-like" class="post-component-bar-button">
                        <img src="/assets/imgs/like.svg" class="post-component-bar-button-logo">
                        <div id="post-{}-like-content" class="post-component-bar-button-content">{}</div>
                    </div>
                    <div id="post-{}-repost" class="post-component-bar-button">
                        <img src="/assets/imgs/Repost.svg" class="post-component-bar-button-logo">
                        <div id="post-{}-repost-content" class="post-component-bar-button-content">{}</div>
                    </div>
                    <a href="/post/{}" class="post-component-bar-button">
                        <img src="/assets/imgs/Comment.svg" class="post-component-bar-button-logo">
                        <div class="post-component-bar-button-content">{}</div>
                    </a>
                </div>
            </div>
        "#,
            post_header,
            account_info.address,
            account_info.photo,
            account_info.address,
            account_info.name,
            account_info.address,
            post_text_class,
            post.content,
            post.id,
            post.id,
            post.like_count,
            post.id,
            post.id,
            post.repost_count,
            post.id,
            post.comments_count
        ));
        Ok(html)
    }

    fn register_events(&self, ctx: Arc<ApplicationContext>) -> Result<()> {
        self.like_event(ctx.clone())?;
        self.repost_event(ctx)?;
        Ok(())
    }
}
