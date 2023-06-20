use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use ntex::web::{self, middleware, App, Error, HttpResponse};
use std::sync::Arc;

use crate::schema::{create_schema, Schema};

mod schema;

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphql", Some("http://127.0.0.1:8080/graphql"));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn graphql(
    st: web::types::State<Arc<Schema>>,
    data: web::types::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let st = (*st).clone();
    let result = web::block(move || {
        let res = data.execute_sync(&st, &());
        serde_json::to_string(&res)
    })
    .await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(result))
}

pub fn register(config: &mut web::ServiceConfig) {
    let schema = Arc::new(create_schema());
    config
        .state(schema)
        .route("/graphql", web::post().to(graphql))
        .route("/graphiql", web::get().to(graphiql));
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8080";
    println!("Running at {address}");
    web::server(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(register)
    })
    .bind(address)?
    .run()
    .await
}
