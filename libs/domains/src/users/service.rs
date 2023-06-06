use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Write};
use std::sync::Arc;
use async_graphql::async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait};
use convention_ninja_database::users::Entity;
use crate::users::model::User;
use crate::users::mutations::OnboardInput;

#[derive(Debug)]
pub enum UsersErr {
    UserNotFound,
    InternalError {
        cause: String,
    },
}

impl Display for UsersErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UsersErr::UserNotFound => f.write_str("User was not found"),
            UsersErr::InternalError { .. } => f.write_str("There was an internal error, please try again"),
        }
    }
}

impl Error for UsersErr {}

#[async_trait]
pub trait UsersService: Sized {
    type Error: Error;
    async fn get(&self, id: &str) -> Result<User, Self::Error>;
    async fn create(&self, input: &OnboardInput) -> Result<User, Self::Error>;
}

pub struct DefaultUsersService {
    db: Arc<DatabaseConnection>,
}

impl DefaultUsersService {
    pub fn new(db: &Arc<DatabaseConnection>) -> Self {
        Self {
            db: db.clone()
        }
    }
}

#[async_trait]
impl UsersService for DefaultUsersService {
    type Error = UsersErr;
    async fn get(&self, id: &str) -> Result<User, Self::Error> {
        let query = Entity::find_by_id(id.to_owned());

        let user = query.one(&*self.db).await;
        if user.is_err() {
            return Err(UsersErr::InternalError {
                cause: user.unwrap_err().to_string()
            });
        };
        let user = user.unwrap();
        match user {
            Some(u) => Ok(User::from(u)),
            None => Err(UsersErr::UserNotFound)
        }
    }

    async fn create(&self, input: &OnboardInput) -> Result<User, Self::Error> {
        todo!()
    }
}