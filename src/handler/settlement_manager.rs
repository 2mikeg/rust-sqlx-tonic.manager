use crate::settlement_manager::settlement_crud_server::{SettlementCrud};
use crate::settlement_manager::{Settlement, SettlementCreate, SettlementGet};
use sqlx::postgres::{PgPool};
use tonic::{Request, Response, Status};
use uuid::Uuid;
use crate::model;
use crate::utils::{native_dt_to_timestamp};

pub mod settlement_manager {
	tonic::include_proto!("settlement_manager");
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
impl SettlementCrud for NewCas {
	async fn create(
		&self,
		request: Request<SettlementCreate>,
	) -> Result<Response<Settlement>, Status> {
		let req: SettlementCreate = request.into_inner();


		let insert_settlement_query = sqlx::query_as!(
			model::settlement_manager::DbSettlement,
			"INSERT INTO settlements (service_id, quantity, price, amount) VALUES ($1, $2, $3, $4) RETURNING id, service_id, quantity, price, amount, created_at;",
			Uuid::parse_str(&req.service_id).unwrap(),
			req.quantity,
			req.price,
			req.amount,
		).fetch_one(&self.pg_pool).await;


		if insert_settlement_query.is_err() {
			Status::internal("Can't insert record into settlement table because query failed.");
		}

		let db_settlement = insert_settlement_query.unwrap();

		let settlement = Settlement {
			id: db_settlement.id,
			service_id: db_settlement.service_id,
			created_at: native_dt_to_timestamp(db_settlement.created_at),
			quantity: db_settlement.quantity,
			price: db_settlement.price,
			amount: db_settlement.amount,
		};

		Ok(Response::new(settlement))
	}

	async fn get(
		&self,
		request: Request<SettlementGet>,
	) -> Result<Response<Settlement>, Status> {


		let req = request.into_inner();

		let get_settlement_query = sqlx::query_as!(
			model::settlement_manager::DbSettlement,
			"SELECT * FROM settlements WHERE id=($1);",
			Uuid::parse_str(&req.id).unwrap()
		).fetch_one(&self.pg_pool).await;

		if get_settlement_query.is_err() {
			Status::internal("Can't found record into settlement table because query failed.");
		}

		let db_settlement = get_settlement_query.unwrap();

		let settlement = Settlement{
			id: db_settlement.id,
			service_id: db_settlement.service_id,
			quantity: db_settlement.quantity,
			price: db_settlement.price,
			amount: db_settlement.amount,
			created_at: native_dt_to_timestamp(db_settlement.created_at),
		};

		Ok(Response::new(settlement))
	}
}