use super::keys;
use crate::clients::blawgd_client::{
    query_client, AccountInfo, GetPostsByAccountRequest, GetPostsByParentPostRequest, GetRequest,
    GetResponse, GetTimelineRequest, Post, PostView,
};
use crate::clients::verification_client::helpers::convert_tm_to_ics_merkle_proof;
use crate::clients::verification_client::proof::{verify_membership, verify_non_membership};
use crate::clients::verification_client::VerificationClient;
use anyhow::anyhow;
use anyhow::Result;
use std::collections::HashMap;

const PER_PAGE: u64 = 30;
const TIMELINE_PER_PAGE: u64 = 5;

impl VerificationClient {
    pub fn with_prefetch(&mut self, root: Vec<u8>, prefetch: GetResponse) {
        self.prefetch = Some((root, prefetch))
    }

    pub async fn get_latest_block_root_height(&self) -> Result<(Vec<u8>, u64)> {
        let lb = self
            .lc
            .supervisor
            .read()
            .await
            .latest_trusted()
            .ok_or(anyhow!("could not get latest trusted light block"))?;
        let mut height = lb.signed_header.header.height.value() - 1;
        let root = lb.signed_header.header.app_hash.value();
        let verify = self.verify.clone();

        if !verify {
            height = 0;
        }

        Ok((root, height))
    }

    pub async fn get(&self, keys: Vec<String>) -> Result<HashMap<String, Option<Vec<u8>>>> {
        let verify = self.verify.clone();
        let (mut root, height) = self.get_latest_block_root_height().await?;

        let data: HashMap<String, Vec<u8>>;
        let proofs: HashMap<String, Vec<u8>>;
        if self.prefetch.is_none() {
            let resp = query_client::QueryClient::new(self.client.clone())
                .get(GetRequest {
                    height,
                    keys: keys.clone(),
                })
                .await?
                .into_inner();

            data = resp.data;
            proofs = resp.proofs;
        } else {
            let (given_root, resp) = self.prefetch.as_ref().unwrap();
            let mut pre_data: HashMap<String, Vec<u8>> = HashMap::new();
            let mut pre_proofs: HashMap<String, Vec<u8>> = HashMap::new();
            for key in keys.iter() {
                pre_data.insert(
                    key.clone(),
                    resp.data
                        .get(key.clone().as_str())
                        .ok_or(anyhow!(
                            "given key {} does not exist in prefetched data",
                            key.clone()
                        ))?
                        .clone(),
                );
                pre_proofs.insert(
                    key.clone(),
                    resp.proofs
                        .get(key.clone().as_str())
                        .ok_or(anyhow!(
                            "given key {} does not exist in prefetched data",
                            key.clone()
                        ))?
                        .clone(),
                );
            }

            data = pre_data;
            proofs = pre_proofs;
            root = given_root.clone();
        }

        if verify {
            for key in keys {
                let val = data
                    .get(&key)
                    .ok_or(anyhow!("did not get data for key {}", key))?
                    .clone();

                let proof = proofs
                    .get(&key)
                    .ok_or(anyhow!("did not get proof for key {}", key))?
                    .clone();
                let proof: tendermint_proto::crypto::ProofOps =
                    prost_2::Message::decode(proof.as_slice())?;
                let proof = convert_tm_to_ics_merkle_proof(proof)?;

                if val.is_empty() {
                    verify_non_membership(proof, root.as_slice(), key.as_bytes())?;
                } else {
                    verify_membership(proof, root.as_slice(), key.as_bytes(), val.as_slice())?;
                }
            }
        }
        let result: HashMap<String, Option<Vec<u8>>> = data
            .iter()
            .map(|(k, v)| {
                let k = k.clone();
                if v.is_empty() {
                    (k, None)
                } else {
                    (k, Some(v.clone()))
                }
            })
            .collect();
        Ok(result)
    }

    pub async fn get_proto<T: prost::Message + std::default::Default>(
        &self,
        keys: Vec<String>,
    ) -> Result<HashMap<String, Option<T>>> {
        let data = self.get(keys).await?;
        let mut result: HashMap<String, Option<T>> = HashMap::new();
        for (k, v) in data {
            if v.is_some() {
                let v: T = prost::Message::decode(v.unwrap().clone().as_slice())?;
                result.insert(k, Some(v));
            } else {
                result.insert(k, None);
            }
        }
        Ok(result)
    }

    pub async fn get_account_info(&self, address: String) -> Result<AccountInfo> {
        let account_info = self
            .get_account_infos(vec![address.clone()])
            .await?
            .first()
            .ok_or(anyhow!("could not get account info for {}", address))?
            .clone();
        Ok(account_info)
    }

    pub async fn get_account_infos(&self, addresses: Vec<String>) -> Result<Vec<AccountInfo>> {
        let mut key_to_address: HashMap<String, String> = HashMap::new();
        let address_to_account_info = self
            .get_proto::<AccountInfo>(
                addresses
                    .iter()
                    .map(|a| {
                        let k = keys::account_info_key(a.clone());
                        key_to_address.insert(k.clone(), a.clone());
                        k
                    })
                    .collect(),
            )
            .await?;

        let mut account_infos: Vec<AccountInfo> = Vec::new();
        for (key, account_info) in address_to_account_info {
            let address = key_to_address
                .get(key.as_str())
                .unwrap_or(&"".to_string())
                .clone();
            let account_info = account_info.unwrap_or(AccountInfo {
                address: address.clone(),
                name: "".to_string(),
                photo: "".to_string(),
                following_count: 0,
                followers_count: 0,
                post_count: 0,
            });
            account_infos.push(normalize_account_info(account_info, address))
        }

        Ok(account_infos)
    }

    pub async fn get_following_list(&self, address: String) -> Result<Vec<String>> {
        let following_list_key = keys::following_key(address);
        let following_list_raw = self
            .get(vec![following_list_key.clone()])
            .await?
            .get(&following_list_key)
            .ok_or(anyhow!(
                "unexpected! did not get data for key {}",
                following_list_key
            ))?
            .clone();

        if following_list_raw.is_some() {
            Ok(String::from_utf8(following_list_raw.unwrap())?
                .split(",")
                .map(|s| String::from(s.clone()))
                .collect())
        } else {
            Ok(Vec::new())
        }
    }
    async fn get_post_by_account(&self, address: String, mut page: u64) -> Result<Vec<PostView>> {
        let account_info = self.get_account_info(address.clone()).await?;
        let mut keys: Vec<String> = Vec::new();
        if account_info.post_count == 0 {
            return Ok(Vec::new());
        }

        if page == 0 {
            page = 1;
        }

        let pagination = pagination(1, account_info.post_count, page, PER_PAGE);
        if pagination.is_err() {
            return Ok(Vec::new());
        }
        let (min, max) = pagination?;
        for i in (min..max + 1).rev() {
            keys.push(keys::user_post_key(address.clone(), i.to_string()))
        }
        let post_ids: Result<Vec<String>, _> = self
            .get(keys)
            .await?
            .values()
            .map(|b| -> Result<String> {
                Ok(String::from_utf8(
                    b.clone().ok_or(anyhow!("could not get a user post"))?,
                )?)
            })
            .collect();
        self.get_posts(post_ids?).await
    }

    pub async fn get_post_by_parent_post_prefetch(
        &self,
        parent_post_id: String,
        page: u64,
    ) -> Result<Vec<PostView>> {
        let (root, height) = self.get_latest_block_root_height().await?;
        let resp = query_client::QueryClient::new(self.client.clone())
            .get_posts_by_parent_post(GetPostsByParentPostRequest {
                height: height as i64,
                page: page as i64,
                per_page: PER_PAGE as i64,
                parent_post: parent_post_id.clone(),
            })
            .await?
            .into_inner();
        let mut vc = self.clone();
        vc.with_prefetch(root, resp);

        Ok(vc.get_post_by_parent_post(parent_post_id, page).await?)
    }

    pub async fn get_post_by_account_prefetch(
        &self,
        address: String,
        page: u64,
    ) -> Result<Vec<PostView>> {
        let (root, height) = self.get_latest_block_root_height().await?;
        let resp = query_client::QueryClient::new(self.client.clone())
            .get_posts_by_account(GetPostsByAccountRequest {
                height: height as i64,
                page: page as i64,
                per_page: PER_PAGE as i64,
                address: address.clone(),
            })
            .await?
            .into_inner();
        let mut vc = self.clone();
        vc.with_prefetch(root, resp);

        Ok(vc.get_post_by_account(address, page).await?)
    }

    pub async fn get_timeline_prefetch(
        &self,
        address: String,
        page: u64,
    ) -> Result<Vec<PostView>> {
        let (root, height) = self.get_latest_block_root_height().await?;
        let resp = query_client::QueryClient::new(self.client.clone())
            .get_timeline(GetTimelineRequest {
                height: height as i64,
                page: page as i64,
                per_page: TIMELINE_PER_PAGE as i64,
                address: address.clone(),
            })
            .await?
            .into_inner();
        let mut vc = self.clone();
        vc.with_prefetch(root, resp);

        Ok(vc.get_timeline(address, page).await?)
    }

    async fn get_post_by_parent_post(
        &self,
        parent_post_id: String,
        mut page: u64,
    ) -> Result<Vec<PostView>> {
        let parent_post = self.get_post(parent_post_id.clone()).await;
        if parent_post.is_err() {
            return Ok(Vec::new());
        }
        let parent_post = parent_post?;
        if parent_post.comments_count == 0 {
            return Ok(Vec::new());
        }
        let mut keys: Vec<String> = Vec::new();

        if page == 0 {
            page = 1;
        }

        let pagination = pagination(1, parent_post.comments_count, page, PER_PAGE);
        if pagination.is_err() {
            return Ok(Vec::new());
        }
        let (min, max) = pagination?;
        for i in (min..max + 1).rev() {
            keys.push(keys::subpost_key(parent_post.id.clone(), i.to_string()))
        }
        let post_ids: Result<Vec<String>, _> = self
            .get(keys)
            .await?
            .values()
            .map(|b| -> Result<String> {
                Ok(String::from_utf8(b.clone().ok_or(anyhow!(
                    "could not get sub post for parent post {}",
                    parent_post_id
                ))?)?)
            })
            .collect();
        self.get_posts(post_ids?).await
    }

    pub async fn get_posts(&self, ids: Vec<String>) -> Result<Vec<PostView>> {
        let keys: Vec<String> = ids.iter().map(|id| keys::post_key(id.clone())).collect();
        let posts: Result<Vec<Post>, _> = self
            .get_proto::<Post>(keys)
            .await?
            .values()
            .filter(|p| p.is_some())
            .map(|p| p.clone().ok_or(anyhow!("could not get post")))
            .collect();
        let posts = posts?;
        let mut account_info_keys = Vec::new();
        for post in posts.clone() {
            account_info_keys.push(keys::account_info_key(post.creator.clone()));
            if post.repost_parent.is_some() {
                account_info_keys.push(keys::account_info_key(
                    post.repost_parent.unwrap().creator.clone(),
                ))
            }
        }
        let account_infos: HashMap<String, AccountInfo> = self
            .get_proto::<AccountInfo>(account_info_keys)
            .await?
            .iter()
            .filter(|(_, v)| v.is_some())
            .map(|(k, v)| (k.clone(), v.as_ref().unwrap().clone()))
            .collect();

        let mut post_views = posts_to_post_views(account_infos, posts);

        post_views.sort_by(|a, b| {
            let id1: u64 = a.id.parse().unwrap_or(0);
            let id2: u64 = b.id.parse().unwrap_or(0);
            id2.cmp(&id1)
        });

        Ok(post_views)
    }

    pub async fn get_post(&self, id: String) -> Result<PostView> {
        let post_view = self
            .get_posts(vec![id.clone()])
            .await?
            .first()
            .ok_or(anyhow!("could not get post with id {}", id))?
            .clone();
        Ok(post_view)
    }

    async fn get_timeline(&self, address: String, mut page: u64) -> Result<Vec<PostView>> {
        let followings = self.get_following_list(address).await?;
        let mut key_to_address: HashMap<String, String> = HashMap::new();
        for addr in followings.clone() {
            key_to_address.insert(keys::account_info_key(addr.clone()), addr.clone());
        }
        let account_info_keys: Vec<String> = followings
            .iter()
            .map(|v| keys::account_info_key(v.clone()))
            .collect();
        let account_infos = self.get_proto::<AccountInfo>(account_info_keys).await?;
        let mut user_post_keys: Vec<String> = Vec::new();

        if page == 0 {
            page = 1;
        }

        for (k, info) in account_infos {
            if info.is_none() {
                continue;
            }
            let info = info.unwrap();
            let post_count = info.post_count;
            if post_count == 0 {
                continue;
            }

            let pagination = pagination(1, post_count, page, TIMELINE_PER_PAGE);
            if pagination.is_err() {
                continue;
            }
            let (min, max) = pagination?;
            for i in min..max + 1 {
                user_post_keys.push(keys::user_post_key(
                    key_to_address
                        .get(&k.clone())
                        .cloned()
                        .ok_or(anyhow!("couldnt find address for key {}", k))?,
                    i.to_string(),
                ))
            }
        }

        let post_keys: Result<Vec<String>, _> = self
            .get(user_post_keys)
            .await?
            .values()
            .map(|v| -> Result<String> {
                Ok(String::from_utf8(
                    v.clone().ok_or(anyhow!("could not get sub post"))?,
                )?)
            })
            .collect();

        self.get_posts(post_keys?).await
    }
}

pub fn posts_to_post_views(
    account_infos: HashMap<String, AccountInfo>,
    posts: Vec<Post>,
) -> Vec<PostView> {
    let mut post_views: Vec<PostView> = Vec::new();
    for p in posts {
        let p = p.clone();
        let mut account_info = account_infos
            .get(&keys::account_info_key(p.creator.clone()))
            .cloned()
            .unwrap_or(AccountInfo {
                address: "".to_string(),
                name: "".to_string(),
                photo: "".to_string(),
                following_count: 0,
                followers_count: 0,
                post_count: 0,
            });
        if !p.creator.is_empty() {
            account_info = normalize_account_info(account_info, p.creator.clone());
        }

        let mut repost_parent_view: Option<Box<PostView>> = None;
        if p.repost_parent.is_some() {
            let p = p.repost_parent.unwrap().clone();
            let mut account_info = account_infos
                .get(&keys::account_info_key(p.creator.clone()))
                .cloned()
                .unwrap_or(AccountInfo {
                    address: "".to_string(),
                    name: "".to_string(),
                    photo: "".to_string(),
                    following_count: 0,
                    followers_count: 0,
                    post_count: 0,
                });
            if !p.creator.is_empty() {
                account_info = normalize_account_info(account_info, p.creator.clone());
            }
            repost_parent_view = Some(Box::new(PostView {
                id: p.id,
                creator: Some(account_info),
                content: p.content,
                parent_post: p.parent_post,
                comments_count: p.comments_count,
                like_count: p.like_count,
                repost_count: p.repost_count,
                repost_parent: None,
            }))
        }

        post_views.push(PostView {
            id: p.id,
            creator: Some(account_info),
            content: p.content,
            parent_post: p.parent_post,
            comments_count: p.comments_count,
            like_count: p.like_count,
            repost_count: p.repost_count,
            repost_parent: repost_parent_view,
        })
    }

    post_views
}

pub fn normalize_account_info(mut account_info: AccountInfo, address: String) -> AccountInfo {
    account_info.address = address.clone();
    if account_info.photo.is_empty() {
        account_info.photo = "/assets/imgs/profile.jpeg".into();
    }
    if account_info.name.is_empty() {
        let name_suffix: String = address.chars().skip(address.len() - 5).take(5).collect();
        account_info.name = format!("anon{}", name_suffix);
    }
    account_info
}

pub fn pagination(abs_min: u64, abs_max: u64, page: u64, per_page: u64) -> Result<(u64, u64)> {
    let mut max = abs_max;
    let mut min = abs_min;
    if max <= (per_page * (page - 1)) {
        return Err(anyhow!(
            "page number {} to high, not enough pages in collection",
            page
        ));
    }
    if max > per_page {
        max = max - (per_page * (page - 1));
        if max > per_page {
            min = max + 1 - per_page
        }
    };

    Ok((min, max))
}
