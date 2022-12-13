use crate::components::post::PostComponent;
use crate::components::scroll_event::{reg_scroll_event, PageState, ParentPostGetter};
use crate::components::Component;
use crate::context::ApplicationContext;
use anyhow::Result;
use async_lock::RwLock;
use prost::alloc::sync::Arc;

pub struct PostPage {
    nav_bar: Box<dyn Component>,
    main_post: Box<PostComponent>,
    post_creator: Option<Box<dyn Component>>,
    posts: Box<[Box<dyn Component>]>,
    state: Arc<RwLock<PageState>>,
}

impl PostPage {
    pub fn new(
        nav_bar: Box<dyn Component>,
        main_post: Box<PostComponent>,
        post_creator: Option<Box<dyn Component>>,
        posts: Box<[Box<dyn Component>]>,
    ) -> Box<PostPage> {
        Box::new(PostPage {
            nav_bar,
            main_post,
            post_creator,
            posts,
            state: Arc::new(RwLock::new(PageState { page: 1 })),
        })
    }
}

impl super::Component for PostPage {
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
        {}
    </div>
    <div class="secondary-column"></div>
</div>
</div>
"#,
            self.nav_bar.to_html()?,
            self.main_post.to_html()?,
            post_creator,
            posts
        ));
        Ok(html)
    }

    fn register_events(&self, ctx: Arc<ApplicationContext>) -> Result<()> {
        self.main_post.register_events(ctx.clone())?;
        for post in self.posts.iter() {
            post.register_events(ctx.clone())?;
        }
        self.nav_bar.register_events(ctx.clone())?;
        if self.post_creator.is_some() {
            self.post_creator
                .as_ref()
                .unwrap()
                .register_events(ctx.clone())?;
        }

        reg_scroll_event(
            self.state.clone(),
            ctx,
            ParentPostGetter {
                parent_post: self.main_post.post.id.clone(),
            },
        )?;

        Ok(())
    }
}
