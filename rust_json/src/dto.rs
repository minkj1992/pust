use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct User<'a> {
    id: i64,
    name: &'a str,
    surname: &'a str,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Article<'a> {
    id: i64,
    author: User<'a>,
    title: &'a str,
    body: &'a str,
    #[serde(default)]
    tags: Vec<Tag<'a>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Tag<'a> {
    id: i64,
    name: &'a str,
}

#[cfg(test)]
mod tests {
    use crate::consts;

    use super::*;

    #[test]
    fn should_parse_json_to_articles() {
        let expect: Vec<Article> = Vec::from([
            Article { 
                id: 1, 
                author: User { id: 4, name: "seray", surname: "uzgur" }, 
                title: "Rust Rocks!", 
                body: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero.", 
                tags: Vec::from([
                    Tag { id: 1, name: "tech" }, 
                    Tag { id: 2, name: "web" }]) },
            Article { id: 2, author: User { id: 5, name: "kamil", surname: "bukum" }, title: "TypeScript is Awesome", body: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero.", tags: Vec::from([Tag { id: 1, name: "tech" }, Tag { id: 2, name: "web" }]) }, 
            Article { id: 3, author: User { id: 6, name: "hasan", surname: "mumin" }, title: "KendoUI Rocks!", body: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero.", tags: Vec::from([Tag { id: 1, name: "tech" }, Tag { id: 2, name: "web" }]) }
            ]);

        let given: Vec<Article> = serde_json::from_str(consts::RAW_ARTICLES).unwrap();
        
        assert_eq!(given, expect);
    }

    #[test]
    fn should_parse_json_to_users() {
        let expect: Vec<User> = Vec::from([
            User { id: 1, name: "leoo", surname: "je" }, 
            User { id: 2, name: "minwook", surname: "je" }, 
            User { id: 3, name: "john", surname: "park" }, 
            User { id: 4, name: "seray", surname: "uzgur" }, 
            User { id: 5, name: "kamil", surname: "bukum" }, 
            User { id: 6, name: "hasan", surname: "mumin" },
        ]);

        let given: Vec<User> = serde_json::from_str(consts::RAW_USERS).unwrap();
        
        assert_eq!(given, expect);
    }
}
