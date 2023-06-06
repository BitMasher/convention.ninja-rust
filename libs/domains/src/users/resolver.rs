use async_graphql::{Context, Object, Result};
use chrono::Utc;
use crate::users::mutations::OnboardInput;

use super::{
    model::User
};

#[derive(Default)]
pub struct UsersQuery {}

#[Object]
impl UsersQuery {
    async fn me(&self, _ctx: &Context<'_>) -> Result<Option<User>> {
        Ok(Some(User::default()))
    }
}

#[derive(Default)]
pub struct UsersMutation {}

#[Object]
impl UsersMutation {
    async fn onboard(&self, _ctx: &Context<'_>, _input: OnboardInput) -> Result<Option<User>> {
        Ok(Some(User {
            id: "id".to_owned(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            display_name: match _input.display_name {
                Some(d) => d,
                None => _input.name.clone()
            },
            name: Some(_input.name),
            dob: Some(_input.dob),
            contact_email: Some(_input.contact_email),
            city: Some(_input.city),
            state: Some(_input.state),
        }))
    }
}