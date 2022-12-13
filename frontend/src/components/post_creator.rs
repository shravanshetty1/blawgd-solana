use crate::clients::blawgd_client::{MsgCreatePost, MSG_TYPE_CREATE_POST};
use crate::context::ApplicationContext;
use crate::task;
use anyhow::anyhow;
use anyhow::Result;
use gloo::events;
use std::sync::Arc;
use wasm_bindgen::JsCast;

pub struct PostCreator {
    button_text: String,
    parent_post: String,
}

impl PostCreator {
    pub fn new(parent_post: String) -> Box<PostCreator> {
        Box::new(PostCreator {
            parent_post,
            button_text: "Post".into(),
        })
    }

    pub fn set_button_text(&mut self, text: &str) {
        self.button_text = text.into();
    }
}

impl super::Component for PostCreator {
    fn to_html(&self) -> Result<String> {
        Ok(String::from(format!(
            r#"
        <div class="post-creator">
            <textarea id="post-creator-input" class="post-creator-input" placeholder="Whats happening?"></textarea>
            <div class="post-creator-buttons">
                <div id="post-creator-button" class="post-creator-button-post">
                    {}
                </div>
            </div>
        </div>"#,
            self.button_text
        )))
    }

    fn register_events(&self, ctx: Arc<ApplicationContext>) -> Result<()> {
        let document = ctx.window.document()?;
        let post_creator_button = document.get_element_by_id("post-creator-button")?.inner();
        let parent_post = self.parent_post.clone();
        events::EventListener::new(&post_creator_button, "click", move |_| {
            let document = document.clone();
            let ctx = ctx.clone();
            let parent_post = parent_post.clone();
            task::spawn_local(async move {
                let post_content: String = document
                    .get_element_by_id("post-creator-input")?
                    .inner()
                    .dyn_ref::<web_sys::HtmlTextAreaElement>()
                    .unwrap()
                    .value();
                let msg = MsgCreatePost {
                    creator: ctx
                        .session
                        .clone()
                        .ok_or(anyhow!("user not logged in"))?
                        .account_info
                        .address,
                    content: post_content,
                    parent_post,
                };

                let resp = ctx
                    .client
                    .cosmos
                    .broadcast_tx(ctx.store.get_wallet()?, MSG_TYPE_CREATE_POST, msg)
                    .await?
                    .into_inner();
                crate::logger::console_log(resp.tx_response.unwrap().raw_log.as_str());
                ctx.store.set_should_verify(false)?;
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
