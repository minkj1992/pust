mod consts;
mod dto;

// User: Article = 1:n
// Tag: Article = n:m
fn main() {
    let articles: Vec<dto::Article> = serde_json::from_str(consts::RAW_ARTICLES).unwrap();
    let users: Vec<dto::User> = serde_json::from_str(consts::RAW_USERS).unwrap();

    println!("{:#?}", articles);
    println!("{:#?}", users);
}
