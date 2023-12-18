use sqlx::postgres::{PgPoolOptions};
use tonic::{transport::Server};
use dotenv::dotenv;
use settlement_manager::settlement_crud_server::SettlementCrudServer;

use log;

mod conf;
mod handler;
mod model;
mod utils;

pub mod settlement_manager {
    tonic::include_proto!("settlement_manager");
}


fn get_conn_string(conf: conf::Conf) -> String {
    let conn_string = format!(
        "postgres://{}:{}@{}/{}",
        conf.db_user, conf.db_password, conf.db_host, conf.db_name
    );

    conn_string
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    env_logger::builder().filter_level(log::LevelFilter::Info).init();

    dotenv().ok();
    let conf = conf::load_env();


    let conn_string = get_conn_string(conf);

    let pg_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&conn_string)
        .await?;

    sqlx::migrate!()
        .run(&pg_pool)
        .await?;

    let addr = "[::1]:50051".parse().unwrap();
    let settlement_manager = handler::settlement_manager::NewSettlementManager::new(pg_pool);

    log::info!("Server started {}! Running on {}", "\u{1F680}",addr);

    Server::builder()
        .add_service(SettlementCrudServer::new(settlement_manager))
        .serve(addr)
        .await?;

    Ok(())
}
