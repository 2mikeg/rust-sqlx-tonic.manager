use cas::cas_server::{Cas, CasServer};
use cas::{EsSetlement, EsSetlementBase, EsSettlementCreate, EsSettlementGet};
use tonic::{transport::Server, Request, Response, Status};

pub mod cas {
    tonic::include_proto!("cas");
}

#[derive(Debug, Default)]
pub struct NewCas {}

#[tonic::async_trait]
impl Cas for NewCas {
    async fn create(
        &self,
        request: Request<EsSettlementCreate>,
    ) -> Result<Response<EsSetlement>, Status> {
        let req = request.into_inner();
        let req_settlement = req.settlement.unwrap();

        let resp = cas::EsSetlement {
            id: "asd-asd-asd".parse().unwrap(),
            settlement: Some(EsSetlementBase {
                service_id: req_settlement.service_id,
                quantity: req_settlement.quantity,
                price: req_settlement.quantity,
                amount: req_settlement.amount,
            }),
            created_at: "2023-01-01".parse().unwrap(),
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
            settlement: Some(EsSetlementBase {
                service_id: "asd-asd-asd".parse().unwrap(),
                quantity: 1.0,
                price: 1.0,
                amount: 1.0,
            }),
            created_at: "2023-01-01".parse().unwrap(),
        };

        Ok(Response::new(resp))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let cas: NewCas = NewCas::default();

    Server::builder()
        .add_service(CasServer::new(cas))
        .serve(addr)
        .await?;

    println!("Server started! Running on port: 50051");

    Ok(())
}
