use serde::{
    de::{SeqAccess, Visitor},
    Deserialize, Serialize,
};
use serde_json::Result;

// User: Article = 1:n
// Tag: Article = n:m

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct User {
    id: i64,
    name: String,
    surname: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Article {
    id: i64,
    // TODO: check parser is needed?
    author : User,
    title: String,
    body: String,
    #[serde(deserialize_with = "parse_tags")]
    tags: Vec<Tag>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Tag {
    id: i64,
    name: String,
}

const RAW_USERS: &str = r#"
[
    {
        "id": 1,
        "name": "leoo",
        "surname": "je"
    },
    {
        "id": 2,
        "name": "minwook",
        "surname": "je"
    },
    {
        "id": 3,
        "name": "john",
        "surname": "park"
    },
]
"#;

const RAW_ARTICLES: &str = r#"
[
    {
        "author": {
            "name": "seray",
            "surname": "uzgur"
        },
        "body": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero.",
        "id": 1,
        "tags": [
            {
                "id": 1,
                "name": "tech"
            },
            {
                "id": 2,
                "name": "web"
            }
        ],
        "title": "Rust Rocks!"
    },
    {
        "author": {
            "name": "kamil",
            "surname": "bukum"
        },
        "body": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero.",
        "id": 2,
        "tags": [
            {
                "id": 1,
                "name": "tech"
            },
            {
                "id": 2,
                "name": "web"
            }
        ],
        "title": "TypeScript is Awesome"
    },
    {
        "author": {
            "name": "hasan",
            "surname": "mumin"
        },
        "body": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer nec odio. Praesent libero.",
        "id": 3,
        "tags": [
            {
                "id": 1,
                "name": "tech"
            },
            {
                "id": 2,
                "name": "web"
            }
        ],
        "title": "KendoUI Rocks!"
    }
]
"#;

// https://docs.rs/serde/latest/serde/de/trait.Visitor.html
fn parse_tags<'de, D>(deserializer: D) -> Result<Vec<Tags>, D::Error>
where
    D: Deserializer<'de>,
{
    struct TagParser;
    impl<'de> Visitor<'de> for TagParser {
        type Value = Tag;

        // TODO: check
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("Check Tag struct").
        }

        // TODO: check
        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            println!("Custom tag deserializer");
            let mut prop = Tag {
                ..Default::default()
            };

            let tmp = seq.next_element::<Key>()?;
            if let Some(a) = tmp {
                prop.id = a.value;
            };

            let tmp = seq.next_element::<Val>()?;
            if let Some(b) = tmp {
                prop.name = b.value;
            };

            Ok(prop)
        }
    }

    deserializer.deserialize_any(TagParser {})
}

fn main() {
    let users = serde_json::from_str(RAW_USERS).unwrap();
    let articles = serde_json::from_str(RAW_ARTICLES).unwrap();

}
