use core::option::Option;

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait};
use sea_orm::ActiveValue::Set;
use sea_orm::QueryFilter;

use crate::encryption;
use crate::entities::prelude::User;
use crate::entities::user;

pub struct AuthService;

impl AuthService {
    pub async fn sign_in(
        input: (String, String),
        conn: &DatabaseConnection,
    ) -> Result<user::Model, String> {
        let email = input.0;
        let password = input.1;

        let user_opt = User::find()
            .filter(user::Column::Email.eq(email))
            .one(conn)
            .await
            .unwrap();

        if user_opt.is_none() {
            return Err("".parse().unwrap());
        }
        let user = user_opt.unwrap();

        if encryption::verify_password(password, user.password.clone().unwrap()).await {
            Ok(user)
        } else {
            Err("".parse().unwrap())
        }
    }

    pub async fn sign_up(
        input: (String, String),
        conn: &DatabaseConnection,
    ) -> Result<i64, String> {
        let email = input.0;
        let password = input.1;

        let existing_user = User::find()
            .filter(user::Column::Email.eq(email.as_str()))
            .one(conn)
            .await
            .unwrap();
        if existing_user.is_some() {
            return Err("User already exists".parse().unwrap());
        }

        let hashed = encryption::hash_password(password).await;

        let user = user::ActiveModel {
            email: Set(Option::from(email)),
            password: Set(Option::from(hashed)),
            ..Default::default()
        };

        let user_res = User::insert(user).exec(conn).await.unwrap();

        Ok(user_res.last_insert_id)
    }
}
