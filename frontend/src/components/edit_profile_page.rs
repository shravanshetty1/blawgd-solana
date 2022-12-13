use super::Component;
use crate::clients::blawgd_client;
use crate::clients::blawgd_client::MSG_TYPE_UPDATE_ACCOUNT_INFO;
use crate::context::ApplicationContext;
use crate::task;
use anyhow::anyhow;
use anyhow::Result;
use gloo::events;
use prost::alloc::sync::Arc;
use task::spawn_local;
use wasm_bindgen::JsCast;

pub struct EditProfilePage {
    nav_bar: Box<dyn Component>,
    account_info: Box<dyn Component>,
}

impl EditProfilePage {
    pub fn new(
        nav_bar: Box<dyn Component>,
        account_info: Box<dyn Component>,
    ) -> Box<EditProfilePage> {
        Box::new(EditProfilePage {
            nav_bar,
            account_info,
        })
    }
}

impl Component for EditProfilePage {
    fn to_html(&self) -> Result<String> {
        Ok(String::from(format!(
            r#"
<div class="page">
<div class="page-wrapper">
    {}
    <div class="main-column">
        <div class="account-info-wrapper">
            {}
            <div class="new-account-info">
                <input id="image-field" class="account-info-field" type="text" placeholder="Image here...">
                <input id="name-field" class="account-info-field" type="text" placeholder="Name here...">
                <div class="new-account-info-buttons">
                    <div id="preview-button" class="button">Preview</div>
                    <div id="reset-button" class="button">Reset</div>
                </div>
            </div>
            <div id="update-profile" class="button">Update Profile</div>
        </div>
    </div>
    <div class="secondary-column"></div>
</div>
</div>
"#,
            self.nav_bar.to_html()?,
            self.account_info.to_html()?,
        )))
    }

    fn register_events(&self, ctx: Arc<ApplicationContext>) -> Result<()> {
        self.account_info.register_events(ctx.clone())?;
        self.nav_bar.register_events(ctx.clone())?;

        let document = ctx.window.document()?;
        let preview_button = document.get_element_by_id("preview-button")?.inner();
        events::EventListener::new(&preview_button, "click", move |_| {
            let document = document.clone();
            spawn_local(async move {
                let image_field = document.get_element_by_id("image-field")?;
                let image_link: String = image_field
                    .inner()
                    .dyn_ref::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value();
                let name_field = document.get_element_by_id("name-field")?;
                let name: String = name_field
                    .inner()
                    .dyn_ref::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value();
                document
                    .get_element_by_id("account-info-name")?
                    .set_inner_html(name.as_str());
                document
                    .get_element_by_id("account-info-photo")?
                    .inner()
                    .dyn_ref::<web_sys::HtmlImageElement>()
                    .unwrap()
                    .set_src(image_link.as_str());

                Ok(())
            });
        })
        .forget();

        let document = ctx.window.document()?;
        let reset_button = document.get_element_by_id("reset-button")?.inner();
        let session = ctx.session.clone();
        events::EventListener::new(&reset_button, "click", move |_| {
            let document = document.clone();
            let session = session.clone();
            spawn_local(async move {
                let account_info = session
                    .ok_or(anyhow!("not logged in to reset"))?
                    .account_info;
                document
                    .get_element_by_id("account-info-name")?
                    .set_inner_html(account_info.name.as_str());
                document
                    .get_element_by_id("account-info-photo")?
                    .inner()
                    .dyn_ref::<web_sys::HtmlImageElement>()
                    .unwrap()
                    .set_src(account_info.photo.as_str());

                document
                    .get_element_by_id("image-field")?
                    .inner()
                    .dyn_ref::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .set_value("");
                document
                    .get_element_by_id("name-field")?
                    .inner()
                    .dyn_ref::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .set_value("");

                Ok(())
            });
        })
        .forget();

        let document = ctx.window.document()?;
        let update_profile_button = document.get_element_by_id("update-profile")?.inner();
        events::EventListener::new(&update_profile_button, "click", move |_| {
            let ctx = ctx.clone();
            let document = document.clone();
            spawn_local(async move {
                let photo: String = document
                    .get_element_by_id("image-field")?
                    .inner()
                    .dyn_ref::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value();
                let name: String = document
                    .get_element_by_id("name-field")?
                    .inner()
                    .dyn_ref::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value();

                let msg = blawgd_client::MsgUpdateAccountInfo {
                    creator: ctx
                        .session
                        .as_ref()
                        .ok_or(anyhow!("not logged in"))?
                        .account_info
                        .address
                        .clone(),
                    name,
                    photo,
                };
                ctx.client
                    .cosmos
                    .broadcast_tx(ctx.store.get_wallet()?, MSG_TYPE_UPDATE_ACCOUNT_INFO, msg)
                    .await?;
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
