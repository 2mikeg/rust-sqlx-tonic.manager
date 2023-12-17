use crate::cas::cas_server::{Cas};
use crate::cas::{EsSetlement, EsSettlementCreate, EsSettlementGet};
use sqlx::postgres::{PgPool, PgPoolOptions};
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub mod cas {
    tonic::include_proto!("cas");
}


#[derive(Debug)]
pub struct NewCas {
    pg_pool: PgPool,
}

impl NewCas {
    pub fn new(pg_pool: PgPool) -> Self {
        Self { pg_pool }
    }
}


#[tonic::async_trait]
impl Cas for NewCas {
    async fn create(
        &self,
        request: Request<EsSettlementCreate>,
    ) -> Result<Response<EsSetlement>, Status> {
        let req: EsSettlementCreate = request.into_inner();


        let insert_settlement_query = sqlx::query_as!(
            cas::EsSetlement,
            "INSERT INTO settlements (service_id, quantity, price, amount) VALUES ($1, $2, $3, $4) RETURNING id, service_id, quantity, price, amount, created_at;",
            Uuid::parse_str(&req.service_id).unwrap(),
            req.quantity,
            req.price,
            req.amount,
        )
            .fetch_one(&self.pg_pool)
            .await;

        if insert_settlement_query.is_err() {
            Status::internal("Can't insert record into settlement table because query failed.");
        }

        let ess_settlement = insert_settlement_query.unwrap();

        let resp = EsSetlement {
            id: ess_settlement.id,
            service_id: ess_settlement.service_id,
            created_at: ess_settlement.created_at,
            quantity: ess_settlement.quantity,
            price: ess_settlement.price,
            amount: ess_settlement.amount,
        };

        Ok(Response::new(resp))
    }

    async fn get(
        &self,
        request: Request<EsSettlementGet>,
    ) -> Result<Response<EsSetlement>, Status> {


        let req = request.into_inner();

        let get_settlement_query = sqlx::query_as!(
            EsSetlement,
            "SELECT * FROM settlements WHERE id=($1);",
            req.id,
        )
            .fetch_one(&self.pg_pool)
            .await;

        if get_settlement_query.is_err() {
            Status::internal("Can't found record into settlement table because query failed.");
        }

        let ess_settlement = get_settlement_query.unwrap();

        Ok(Response::new(ess_settlement))
    }
}