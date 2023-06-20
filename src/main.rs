use std::sync::Arc;

use actix_web::{
    guard,
    web::{self, Data},
    HttpResponse, HttpServer,
};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use schema::{ProjectSchema, create_schema};

mod schema;

async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

async fn graphql_handler(
    schema: web::Data<Arc<ProjectSchema>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let st = (*schema).clone();
    st.execute(req.into_inner()).await.into()
}

pub fn register(config: &mut web::ServiceConfig) {
    let schema = Arc::new(create_schema());
    config
        .app_data(Data::new(schema))
        .service(
            web::resource("/graphql")
                .guard(guard::Post())
                .to(graphql_handler),
        )
        .service(
            web::resource("/graphiql")
                .guard(guard::Get())
                .to(graphql_playground),
        );
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let address = ("127.0.0.1", 8080);

    println!("Server running on http://{}:{}", address.0, address.1);
    HttpServer::new(move || actix_web::App::new().configure(register))
        .bind(address)?
        .run()
        .await
}
