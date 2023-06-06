use async_graphql::InputObject;
use chrono::NaiveDate;

#[derive(Clone, Default, Eq, PartialEq, InputObject)]
pub struct OnboardInput {
    pub name: String,
    pub dob: NaiveDate,
    pub display_name: Option<String>,
    pub city: String,
    pub state: String,
    pub contact_email: String,
}