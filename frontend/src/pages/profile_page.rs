use crate::clients::verification_client::VerificationClient;
use crate::components::nav_bar::NavBar;
use crate::components::post::PostComponent;
use crate::components::profile_page::{ButtonType, ProfilePage};
use crate::components::Component;
use crate::context::{ApplicationContext, SessionInfo};
use crate::pages::PageBuilder;
use anyhow::anyhow;
use anyhow::Result;
use futures::future::try_join3;
use std::sync::Arc;

impl PageBuilder {
    pub async fn profile_page(ctx: Arc<ApplicationContext>) -> Result<Box<dyn Component>> {
        let url: String = ctx.window.location().href()?;
        let address = url
            .as_str()
            .strip_prefix(format!("{}/profile/", ctx.host.endpoint()).as_str())
            .ok_or(anyhow!("could not get address from url {}", url.clone()))?
            .to_string();

        let account_info = ctx.client.vc.get_account_info(address.clone());
        let posts = ctx
            .client
            .vc
            .get_post_by_account_prefetch(address.clone(), 1 as u64);
        let is_following =
            is_following(ctx.client.vc.clone(), ctx.session.clone(), address.clone());
        let (account_info, posts, is_following) =
            try_join3(account_info, posts, is_following).await?;

        let session = ctx.session.clone();
        let mut profile_button: Option<ButtonType> = None;
        if session.is_some() {
            if address == session.clone().unwrap().account_info.address {
                profile_button = Some(ButtonType::Edit);
            } else {
                if is_following {
                    profile_button = Some(ButtonType::Unfollow);
                } else {
                    profile_button = Some(ButtonType::Follow)
                }
            }
        }

        let posts: Vec<Box<dyn Component>> = posts
            .iter()
            .map(|p| PostComponent::new(p.clone()) as Box<dyn Component>)
            .collect();
        let nav_bar = NavBar::new(session.clone());
        let profile_page = ProfilePage::new(
            nav_bar,
            account_info.clone(),
            profile_button.clone(),
            posts.into_boxed_slice(),
        );
        Ok(profile_page)
    }
}

pub async fn is_following(
    cl: VerificationClient,
    session: Option<SessionInfo>,
    address2: String,
) -> Result<bool> {
    if session.is_none() {
        return Ok(false);
    }
    let address1 = session.unwrap().account_info.address;
    let followings = cl.get_following_list(address1).await?;

    let mut is_following: bool = false;
    for following in followings {
        if following.to_string() == address2 {
            is_following = true;
        }
    }

    Ok(is_following)
}
