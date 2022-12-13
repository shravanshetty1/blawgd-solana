use crate::context::{ApplicationContext, SessionInfo};
use anyhow::Result;
use std::sync::Arc;

pub struct NavBar {
    session: Option<SessionInfo>,
}

impl NavBar {
    pub fn new(session: Option<SessionInfo>) -> Box<NavBar> {
        Box::new(NavBar { session })
    }
}

impl super::Component for NavBar {
    fn to_html(&self) -> Result<String> {
        let mut account_menu_items: String = String::new();
        let mut login_component: String = String::from(
            r#"
            <a href="/login" class="login-link-component-wrapper">
                <img src="/assets/imgs/profile.jpeg" class="post-component-account-info-image">
                <div class="login-link-component-text">Login/Register</div>
            </a>
            "#,
        );

        if self.session.is_some() {
            let session = self.session.as_ref().unwrap();
            let account_info = session.account_info.clone();
            account_menu_items = String::from(format!(
                r#"
                <a href="/timeline" class="nav-bar-menu-element">
                    <img src="/assets/imgs/clock.svg" class="nav-bar-menu-element-logo"> <div class="nav-bar-menu-element-text">Timeline</div>
                </a>
                <a href="/profile/{}" class="nav-bar-menu-element">
                    <img src="/assets/imgs/User.svg" class="nav-bar-menu-element-logo"> <div class="nav-bar-menu-element-text">Profile</div>
                </a>"#,
                account_info.address
            ));

            let mut login_comp_text = account_info.name.clone();

            if login_comp_text.is_empty() {
                login_comp_text = account_info.address.clone();
            }

            login_component = String::from(format!(
                r#"
            <div class="nav-bar-balance">
                <img src="/assets/imgs/money.svg" class="nav-bar-balance-logo"> <div class="nav-bar-balance-value">{}</div>
            </div>
            <a href="/login" class="login-link-component-wrapper">
                <img src="{}" class="post-component-account-info-image">
                <div class="login-link-component-text">{}</div>
            </a>
            "#,
                session.balance.clone(),
                account_info.photo.clone(),
                login_comp_text
            ));
        }

        let html = String::from(format!(
            r#"
        <div class="nav-bar">
            <a href="/" class="nav-bar-header">
                <img src="/assets/imgs/logo.png">
            </a>
            <div>Censorship resistant blogging platform</div>
            <div class="insecure-notice">Warning! Current version is a prototype and not secure.</div>
            <div class="nav-bar-menu">
                <a href="/" class="nav-bar-menu-element">
                    <img src="/assets/imgs/explore.svg" class="nav-bar-menu-element-logo"> <div class="nav-bar-menu-element-text">Explore</div>
                </a>
                {}
                <a href="https://github.com/shravanshetty1/blawgd" class="nav-bar-menu-element">
                    <img src="/assets/imgs/about.svg" class="nav-bar-menu-element-logo"> <div class="nav-bar-menu-element-text">About</div>
                </a>

            </div>
            {}
        </div>"#,
            account_menu_items, login_component
        ));
        Ok(html)
    }

    fn register_events(&self, _: Arc<ApplicationContext>) -> Result<()> {
        Ok(())
    }
}
