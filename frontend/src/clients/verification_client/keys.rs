pub fn user_post_key(address: String, order: String) -> String {
    format!("up-{}-{}", address, order)
}

pub fn subpost_key(parent_post: String, order: String) -> String {
    format!("sp-{}-{}", parent_post, order)
}

// pub fn post_count_key() -> String {
//     String::from("pc")
// }

pub fn post_key(order: String) -> String {
    format!("p-{}", order)
}

pub fn account_info_key(address: String) -> String {
    format!("ac-{}", address)
}

pub fn following_key(address: String) -> String {
    format!("f-{}", address)
}
