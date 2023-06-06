#![forbid(unsafe_code)]

use std::convert::Infallible;
use std::sync::Arc;
use async_graphql::http::GraphiQLSource;
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use warp::{Filter, Rejection};
use warp::http::{StatusCode};
use warp::reply::html;
use crate::graphql::{create_schema, GraphQLSchema};

pub mod graphql;

pub struct Context {}

pub async fn run(ctx: Arc<Context>) {
    let schema = create_schema(ctx);

    println!("GraphiQL IDE: http://localhost:8080");

    let graphql_post = async_graphql_warp::graphql(schema)
        .and_then(
            |(schema, request): (
                GraphQLSchema,
                async_graphql::Request,
            )| async move {
                Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
            },
        );

    let graphiql = warp::path::end().and(warp::get()).map(|| {
        html(GraphiQLSource::build().endpoint("/").finish())
    });

    let routes = graphiql
        .or(graphql_post)
        .recover(|err: Rejection| async move {
            if let Some(GraphQLBadRequest(err)) = err.find() {
                return Ok::<_, Infallible>(warp::reply::with_status(
                    err.to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }

            Ok(warp::reply::with_status(
                "INTERNAL_SERVER_ERROR".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        });

    warp::serve(routes)
        .run(
            ([0, 0, 0, 0], 8080)
        ).await
}