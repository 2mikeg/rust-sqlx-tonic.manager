use cas::cas_server::{Cas, CasServer};
use cas::{EsSetlement, EsSettlementCreate, EsSettlementGet};
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::{Pool, Postgres};
use tonic::{transport::Server, Request, Response, Status};

mod conf;
pub mod cas {
    tonic::include_proto!("cas");
}

#[derive(Debug)]
pub struct NewCas {
    pg_pool: PgPool,
}

impl NewCas {
    pub fn new(pg_pool: PgPool) -> Self {
        Self { pg_pool: pg_pool }
    }
}

#[tonic::async_trait]
impl Cas for NewCas {
    async fn create(
        &self,
        request: Request<EsSettlementCreate>,
    ) -> Result<Response<EsSetlement>, Status> {
        let req: EsSettlementCreate = request.into_inner();

        let resp = cas::EsSetlement {
            id: "asd-asd-asd".parse().unwrap(),
            service_id: req.service_id,
            created_at: "2023-01-01".parse().unwrap(),
            quantity: req.quantity,
            price: req.price,
            amount: req.amount,
        };

        Ok(Response::new(resp))
    }

    async fn get(
        &self,
        request: Request<EsSettlementGet>,
    ) -> Result<Response<EsSetlement>, Status> {
        let req = request.into_inner();
        let resp = EsSetlement {
            id: req.id,
            service_id: "asd-asd-asd".parse().unwrap(),
            quantity: 1.0,
            price: 1.0,
            amount: 1.0,
            created_at: "2023-01-01".parse().unwrap(),
        };

        Ok(Response::new(resp))
    }
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
    let conf = conf::load_env();

    let conn_string = get_conn_string(conf);

    let pg_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&conn_string)
        .await?;

    let addr = "[::1]:50051".parse().unwrap();
    let cas: NewCas = NewCas::new(pg_pool);

    Server::builder()
        .add_service(CasServer::new(cas))
        .serve(addr)
        .await?;

    println!("Server started! Running on port: 50051");

    Ok(())
}
