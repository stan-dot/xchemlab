#![doc = include_str!("../../README.md")]
#![forbid(unsafe_code)]
pub mod api;
pub mod models;

use api::{schema_builder, RootSchema};
use async_graphql::{extensions::Tracing, http::GraphiQLSource, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use auth::{create_oidc_client, get_authentication_url};
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Extension, Router, Server,
};
use clap::Parser;
use std::{
    fs::File,
    io::Write,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    path::PathBuf,
};

fn setup_api() -> RootSchema {
    schema_builder().extension(Tracing).finish()
}

async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

async fn graphql_handler(schema: Extension<RootSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

fn setup_router<Q, M, S>(schema: Schema<Q, M, S>) -> Router
where
    Q: async_graphql::ObjectType + 'static,
    M: async_graphql::ObjectType + 'static,
    S: async_graphql::SubscriptionType + 'static,
{
    Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .route_service("/ws", GraphQLSubscription::new(schema.clone()))
        .layer(Extension(schema))
}

async fn serve(router: Router, port: u16) {
    let socket_addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port));
    println!("GraphiQL IDE: {}", socket_addr);
    Server::bind(&socket_addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
enum Cli {
    /// Starts a webserver serving the GraphQL API
    Serve(ServeArgs),
    /// Prints the GraphQL API to stdout
    Schema(SchemaArgs),
}

#[derive(Debug, Parser)]
struct ServeArgs {
    /// The port number to serve on.
    #[arg(short, long, default_value_t = 80)]
    port: u16,
}

#[derive(Debug, Parser)]
struct SchemaArgs {
    /// The file path to write the schema to. If not supplied the schema will be printed to stdout.
    #[arg(short, long)]
    path: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let args = Cli::parse();

    let tracing_subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(tracing_subscriber).unwrap();

    match args {
        Cli::Serve(args) => {
            let schema = setup_api();
            let router = setup_router(schema);
            let auth_client = create_oidc_client()
                .await
                .expect("Failed to setup authentication client");
            println!("Authenticate at {}", get_authentication_url(auth_client));
            serve(router, args.port).await;
        }
        Cli::Schema(args) => {
            let schema = setup_api();
            let schema_string = schema.sdl();
            if let Some(path) = args.path {
                let mut file = File::create(path).unwrap();
                file.write_all(schema_string.as_bytes()).unwrap();
            } else {
                println!("{}", schema_string);
            }
        }
    }
}
