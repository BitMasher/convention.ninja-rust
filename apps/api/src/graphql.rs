use std::sync::Arc;
use async_graphql::{EmptySubscription, MergedObject, Schema};
use convention_ninja_domains::users::resolver::{UsersMutation, UsersQuery};
use crate::Context;

#[derive(MergedObject, Default)]
pub struct QuerySchema(UsersQuery);

#[derive(MergedObject, Default)]
pub struct MutationSchema(UsersMutation);

pub type GraphQLSchema = Schema<QuerySchema, MutationSchema, EmptySubscription>;

pub fn create_schema(_ctx: Arc<Context>) -> GraphQLSchema {
    Schema::build(
        QuerySchema::default(),
        MutationSchema::default(),
        EmptySubscription)
        .finish()
}