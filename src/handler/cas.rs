use crate::cas::cas_server::{Cas};
use crate::cas::{EsSetlement, EsSettlementCreate, EsSettlementGet};
use sqlx::postgres::{PgPool};
use tonic::{Request, Response, Status};
use uuid::Uuid;
use crate::model;
use crate::utils::{native_dt_to_timestamp};

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
			model::cas::DbEsSettlement,
			"INSERT INTO settlements (service_id, quantity, price, amount) VALUES ($1, $2, $3, $4) RETURNING id, service_id, quantity, price, amount, created_at;",
			Uuid::parse_str(&req.service_id).unwrap(),
			req.quantity,
			req.price,
			req.amount,
		).fetch_one(&self.pg_pool).await;


		if insert_settlement_query.is_err() {
			Status::internal("Can't insert record into settlement table because query failed.");
		}

		let db_ess_settlement = insert_settlement_query.unwrap();

		let resp = EsSetlement {
			id: db_ess_settlement.id,
			service_id: db_ess_settlement.service_id,
			created_at: native_dt_to_timestamp(db_ess_settlement.created_at),
			quantity: db_ess_settlement.quantity,
			price: db_ess_settlement.price,
			amount: db_ess_settlement.amount,
		};

		Ok(Response::new(resp))
	}

	async fn get(
		&self,
		request: Request<EsSettlementGet>,
	) -> Result<Response<EsSetlement>, Status> {


		let req = request.into_inner();

		let get_settlement_query = sqlx::query_as!(
			model::cas::DbEsSettlement,
			"SELECT * FROM settlements WHERE id=($1);",
			Uuid::parse_str(&req.id).unwrap()
		).fetch_one(&self.pg_pool).await;

		if get_settlement_query.is_err() {
			Status::internal("Can't found record into settlement table because query failed.");
		}

		let db_ess_settlement = get_settlement_query.unwrap();

		let ess_settlement = EsSetlement{
			id: db_ess_settlement.id,
			service_id: db_ess_settlement.service_id,
			quantity: db_ess_settlement.quantity,
			price: db_ess_settlement.price,
			amount: db_ess_settlement.amount,
			created_at: native_dt_to_timestamp(db_ess_settlement.created_at),
		};

		Ok(Response::new(ess_settlement))
	}
}