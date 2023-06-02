mod model;
mod repository;

use crate::model::group::Group;
use crate::repository::Repository;
use async_graphql::{EmptyMutation, EmptySubscription, Object, Request, Response, Schema};
use axum::routing::post;
use axum::{extract::Extension, Json, Router};

struct Query;

#[Object]
impl Query {
    async fn howdy(&self) -> &'static str {
        "hello"
    }

    // async fn members(&self) -> Vec<Member> {
    //     let members = Repository::dummy_data().1;
    //
    // }

    async fn groups(&self) -> Vec<Group> {
        let data = Repository::dummy_data();
        let aaa: Vec<Group> = data
            .0
            .into_iter()
            .map(|n| n.to_group(&data.2, &data.1))
            .collect();
        aaa
    }
}

type MySchema = Schema<Query, EmptyMutation, EmptySubscription>;
async fn graphql_handler(schema: Extension<MySchema>, req: Json<Request>) -> Json<Response> {
    schema.execute(req.0).await.into()
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();
    let app = Router::new()
        .route("/", post(graphql_handler))
        .layer(Extension(schema));

    let server = async {
        axum::Server::bind(&"0.0.0.0:9000".parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap()
    };
    tokio::join!(server);
}
