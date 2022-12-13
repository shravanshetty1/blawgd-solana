use crate::components::scroll_event::{reg_scroll_event, PageState, TimelinePostGetter};
use crate::components::Component;
use crate::context::ApplicationContext;
use anyhow::anyhow;
use anyhow::Result;
use async_lock::RwLock;
use std::sync::Arc;

pub struct TimelinePage {
    nav_bar: Box<dyn Component>,
    post_creator: Option<Box<dyn Component>>,
    posts: Box<[Box<dyn Component>]>,
    state: Arc<RwLock<PageState>>,
}

impl TimelinePage {
    pub fn new(
        nav_bar: Box<dyn Component>,
        post_creator: Option<Box<dyn Component>>,
        posts: Box<[Box<dyn Component>]>,
    ) -> Box<TimelinePage> {
        Box::new(TimelinePage {
            nav_bar,
            post_creator,
            posts,
            state: Arc::new(RwLock::new(PageState { page: 1 })),
        })
    }
}

impl super::Component for TimelinePage {
    fn to_html(&self) -> Result<String> {
        let mut posts: String = String::new();
        for post in self.posts.iter() {
            posts = format!("{}{}", posts, post.to_html()?)
        }

        let mut post_creator: String = String::new();
        if self.post_creator.is_some() {
            post_creator = self.post_creator.as_ref().unwrap().to_html()?;
        }

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
            post_creator,
            posts
        ));
        Ok(html)
    }

    fn register_events(&self, ctx: Arc<ApplicationContext>) -> Result<()> {
        self.nav_bar.register_events(ctx.clone())?;
        if self.post_creator.is_some() {
            self.post_creator
                .as_ref()
                .unwrap()
                .register_events(ctx.clone())?;
        }
        for p in self.posts.iter() {
            p.register_events(ctx.clone())?;
        }

        reg_scroll_event(
            self.state.clone(),
            ctx.clone(),
            TimelinePostGetter {
                address: ctx
                    .session
                    .as_ref()
                    .ok_or(anyhow!("user not logged in"))?
                    .account_info
                    .address
                    .clone(),
            },
        )?;
        Ok(())
    }
}
