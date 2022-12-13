use crate::clients::blawgd_client::AccountInfo;
use crate::context::ApplicationContext;
use anyhow::Result;
use prost::alloc::sync::Arc;

pub struct AccountInfoComp {
    account_info: AccountInfo,
}

impl AccountInfoComp {
    pub fn new(account_info: AccountInfo) -> Box<AccountInfoComp> {
        Box::new(AccountInfoComp { account_info })
    }
}

impl super::Component for AccountInfoComp {
    fn to_html(&self) -> Result<String> {
        let account_info = self.account_info.clone();
        let html = format!(
            r#"
            <div class="account-info">
                <img id="account-info-photo" src="{}" class="account-info-photo">
                <div id="account-info-name" class="account-info-name">{}</div>
                <div class="account-info-address">@{}</div>
                <div class="account-info-follower-info">
                    <a>{} Followers</a>
                    <a href="/followings/{}" class="account-info-followings">{} Following</a>
                </div>
            </div>
"#,
            account_info.photo,
            account_info.name,
            account_info.address,
            account_info.followers_count,
            account_info.address,
            account_info.following_count
        );
        Ok(html)
    }

    fn register_events(&self, _: Arc<ApplicationContext>) -> Result<()> {
        Ok(())
    }
}
