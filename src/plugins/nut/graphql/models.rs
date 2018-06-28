use std::fmt;

use super::super::super::super::oauth::{self, Oauth as OauthUrl};
use chrono::{DateTime, Utc};

#[derive(GraphQLObject, Debug, Serialize)]
pub struct SiteInfo {
    pub locale: String,
    pub title: String,
    pub subhead: String,
    pub keywords: String,
    pub description: String,
    pub copyright: String,
    pub author: Author,
    pub oauth: Vec<Oauth>,
}

#[derive(GraphQLObject, Debug, Deserialize, Serialize)]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(GraphQLObject, Debug, Deserialize, Serialize)]
pub struct Oauth {
    pub name: String,
    pub url: String,
}
impl Oauth {
    pub fn new(cfg: &oauth::Config) -> Vec<Self> {
        let mut items = Vec::new();
        if let Some(ref c) = cfg.line {
            items.push(Self {
                name: c.name().to_string(),
                url: c.authorization_url(&"".to_string(), &"".to_string()),
            });
        }
        items
    }
}

#[derive(GraphQLObject, Debug, Serialize)]
#[graphql(description = "Message for translate")]
pub struct Locale {
    pub id: String,
    pub lang: String,
    pub code: String,
    pub message: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub enum Role {
    Admin,
    Root,
    Member,
    By(String),
}

impl fmt::Display for Role {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::Admin => fmt.write_str("admin"),
            Role::Root => fmt.write_str("root"),
            Role::Member => fmt.write_str("member"),
            Role::By(n) => fmt.write_str(&n),
        }
    }
}
