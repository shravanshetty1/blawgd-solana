use super::Component;
use crate::context::ApplicationContext;
use anyhow::anyhow;
use anyhow::Result;
use prost::alloc::sync::Arc;

pub struct FaucetPage {
    nav_bar: Box<dyn Component>,
    site_key: String,
}

impl FaucetPage {
    pub fn new(nav_bar: Box<dyn Component>, site_key: String) -> Box<FaucetPage> {
        Box::new(FaucetPage { nav_bar, site_key })
    }
}

impl Component for FaucetPage {
    fn to_html(&self) -> Result<String> {
        Ok(String::from(format!(
            r#"
<div class="page">
<div class="page-wrapper">
    {}
    <div id="main" class="main-column">
        <div class="faucet-page">
            <div class="faucet-page-header">
                {}
            </div>
            <div class="h-captcha" data-sitekey="{}" data-callback="captcha" data-theme="dark"></div>
        </div>
    </div>
    <div class="secondary-column"></div>
</div>
</div>
"#,
            self.nav_bar.to_html()?,
            "Complete sign up process by solving the captcha!",
            self.site_key,
        )))
    }

    fn register_events(&self, ctx: Arc<ApplicationContext>) -> Result<()> {
        // render captcha
        let script = ctx
            .window
            .document()?
            .inner()
            .create_element("script")
            .map_err(|_| anyhow!("could not create element"))?;
        script
            .set_attribute("src", "https://js.hcaptcha.com/1/api.js")
            .map_err(|_| anyhow!("could not set attribute"))?;
        ctx.window
            .document()?
            .get_element_by_id("main")?
            .inner()
            .append_child(&script)
            .map_err(|_| anyhow!("could not append child"))?;

        Ok(())
    }
}
