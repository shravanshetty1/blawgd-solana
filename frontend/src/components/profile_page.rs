use super::Component;
use crate::clients::blawgd_client::{
    AccountInfo, MsgFollow, MsgStopFollow, MSG_TYPE_FOLLOW, MSG_TYPE_STOP_FOLLOW,
};
use crate::components::account_info::AccountInfoComp;
use crate::components::scroll_event::{reg_scroll_event, AccountPostGetter, PageState};
use crate::context::ApplicationContext;
use crate::task;
use anyhow::anyhow;
use anyhow::Result;
use async_lock::RwLock;
use gloo::events;
use prost::alloc::sync::Arc;
use task::spawn_local;

pub struct ProfilePage {
    nav_bar: Box<dyn Component>,
    account_info: AccountInfo,
    button: Option<ButtonType>,
    posts: Box<[Box<dyn Component>]>,
    state: Arc<RwLock<PageState>>,
}

#[derive(Clone)]
pub enum ButtonType {
    Edit,
    Follow,
    Unfollow,
}

impl ProfilePage {
    pub fn new(
        nav_bar: Box<dyn Component>,
        account_info: AccountInfo,
        button: Option<ButtonType>,
        posts: Box<[Box<dyn Component>]>,
    ) -> Box<ProfilePage> {
        Box::new(ProfilePage {
            nav_bar,
            account_info,
            button,
            posts,
            state: Arc::new(RwLock::new(PageState { page: 1 })),
        })
    }
}

impl Component for ProfilePage {
    fn to_html(&self) -> Result<String> {
        let mut posts: String = String::new();
        for post in self.posts.iter() {
            posts = format!("{}{}", posts, post.to_html()?)
        }

        let mut button = String::new();
        if self.button.is_some() {
            button = match self.button.as_ref().unwrap() {
                ButtonType::Edit => {
                    r#"<a href="/edit-profile" class="button">Edit Profile</a>"#.into()
                }

                ButtonType::Unfollow => {
                    r#"<a id="follow-toggle" class="button">Unfollow</a>"#.into()
                }
                ButtonType::Follow => r#"<a id="follow-toggle" class="button">Follow</a>"#.into(),
            }
        }

        let account_info_component = String::from(format!(
            r#"
                <div class="account-info-wrapper">
                    {}
                    {}
                </div>
                "#,
            AccountInfoComp::new(self.account_info.clone()).to_html()?,
            button
        ));

        let html = String::from(format!(
            r#"
<div class="page">
<div class="page-wrapper">
    {}
    <div id="main-column" class="main-column">
        {}
        {}
    </div>
    <div class="secondary-column"></div>
</div>
</div>
"#,
            self.nav_bar.to_html()?,
            account_info_component,
            posts
        ));
        Ok(html)
    }

    fn register_events(&self, ctx: Arc<ApplicationContext>) -> Result<()> {
        self.nav_bar.register_events(ctx.clone())?;
        for post in self.posts.iter() {
            post.register_events(ctx.clone())?;
        }

        reg_scroll_event(
            self.state.clone(),
            ctx.clone(),
            AccountPostGetter {
                address: self.account_info.address.clone(),
            },
        )?;

        let button_type = self.button.clone();
        if button_type.is_none() {
            return Ok(());
        }
        let button_type = button_type.unwrap();
        match button_type {
            ButtonType::Edit => return Ok(()),
            _ => {}
        }

        let document = ctx.window.document()?;
        let follow_toggle = document.get_element_by_id("follow-toggle")?.inner();
        let account_info = self.account_info.clone();
        events::EventListener::new(&follow_toggle, "click", move |_| {
            let ctx = ctx.clone();
            let button_type = button_type.clone();
            let account_info = account_info.clone();
            spawn_local(async move {
                let session = ctx.session.clone().unwrap();
                match button_type {
                    ButtonType::Follow => {
                        ctx.client
                            .cosmos
                            .broadcast_tx(
                                ctx.store.get_wallet()?,
                                MSG_TYPE_FOLLOW,
                                MsgFollow {
                                    creator: session.account_info.address.clone(),
                                    address: account_info.address.clone(),
                                },
                            )
                            .await?;
                    }
                    _ => {
                        ctx.client
                            .cosmos
                            .broadcast_tx(
                                ctx.store.get_wallet()?,
                                MSG_TYPE_STOP_FOLLOW,
                                MsgStopFollow {
                                    creator: session.account_info.address.clone(),
                                    address: account_info.address.clone(),
                                },
                            )
                            .await?;
                    }
                }

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
