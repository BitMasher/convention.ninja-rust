use async_graphql::SimpleObject;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use convention_ninja_database::users::Model;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, SimpleObject)]
pub struct User {
    pub id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: Option<String>,
    pub display_name: String,
    pub dob: Option<NaiveDate>,
    pub contact_email: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
}

impl User {
    pub fn authorization_filter(&self) -> Self {
        // TODO: apply polar authorization scripts here
        let mut user = self.clone();
        user.name = None;
        user.dob = None;
        user.contact_email = None;
        user.city = None;
        user.state = None;

        user
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: String::default(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            name: Some(String::default()),
            display_name: String::default(),
            dob: Some(NaiveDate::default()),
            contact_email: Some(String::default()),
            city: Some(String::default()),
            state: Some(String::default()),
        }
    }
}

impl From<Model> for User {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            created_at: value.created_at,
            updated_at: value.updated_at,
            display_name: match value.display_name {
                Some(d) => d,
                None => value.name.clone()
            },
            name: Some(value.name),
            dob:Some(value.dob),
            contact_email:Some(value.contact_email),
            city:Some(value.city),
            state:Some(value.state)
        }
    }
}