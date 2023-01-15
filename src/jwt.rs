use chrono::{Duration, Utc};
use jsonwebtoken::errors::Result;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static TOKEN_SECRET: Lazy<String> =
    Lazy::new(|| std::env::var("JWT_SECRET").expect("jwt secret must set"));

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Claims {
    sub: i64,
    exp: u64,
    //name: String,
    //roles: ?
}

impl Claims {
    fn new(id: i64) -> Self {
        Self {
            sub: id,
            exp: (Utc::now() + Duration::days(30)).timestamp() as u64,
            // name: "Try".to_string(),
            // roles,
        }
    }

    pub fn get_id(&self) -> i64 {
        self.sub
    }

    // pub fn get_name(&self) -> String {
    //     self.name.clone()
    // }

    // pub fn has_role(&self, role: &str) -> bool {
    //     self.roles.contains(&role.to_string())
    // }
}

pub fn encode(id: i64) -> Result<String> {
    jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(id),
        &EncodingKey::from_secret(TOKEN_SECRET.as_bytes()),
    )
}

pub fn decode(token: &str) -> Result<Claims> {
    let result = jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(TOKEN_SECRET.as_ref()),
        &Validation::default(),
    );

    result.map(|token_data| token_data.claims)
}
