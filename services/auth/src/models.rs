use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::users;
use diesel::pg::Pg;
use jsonwebtoken::{EncodingKey, DecodingKey};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub role: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

impl Claims {
    pub fn encode(&self, secret: &[u8]) -> String {
        jsonwebtoken::encode(&jsonwebtoken::Header::default(), self, &EncodingKey::from_secret(secret)).unwrap()
    }

    pub fn decode(token: &str, secret: &[u8]) -> Result<Self, jsonwebtoken::errors::Error> {
        jsonwebtoken::decode::<Claims>(token, &DecodingKey::from_secret(secret), &jsonwebtoken::Validation::default())
            .map(|data| data.claims)
    }
}