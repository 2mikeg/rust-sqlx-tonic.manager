use sqlx::postgres::{PgPoolOptions};
use tonic::{transport::Server};
use dotenv::dotenv;
use cas::cas_server::CasServer;

mod conf;
mod handler;

pub mod cas {
    tonic::include_proto!("cas");
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
    dotenv().ok();
    let conf = conf::load_env();

    let conn_string = get_conn_string(conf);

    let pg_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&conn_string)
        .await?;

    let addr = "[::1]:50051".parse().unwrap();
    let cas= handler::cas::NewCas::new(pg_pool);

    Server::builder()
        .add_service(CasServer::new(cas))
        .serve(addr)
        .await?;

    println!("Server started! Running on port: 50051");

    Ok(())
}
