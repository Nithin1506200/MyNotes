// use payments::bitcoin_server::{Bitcoin, BitcoinServer};
use payments::bitcoin_server::{Bitcoin, BitcoinServer};
use payments::{BtcPaymentRequest, BtcPaymentResponse};
use tonic::{transport::Server, Request, Response, Status};
pub mod payments {
    tonic::include_proto!("payments");
}
#[derive(Debug, Default)]
pub struct BitcoinService {}
#[tonic::async_trait]
impl Bitcoin for BitcoinService {
    async fn send_payments(
        &self,
        request: Request<BtcPaymentRequest>,
    ) -> Result<Response<BtcPaymentResponse>, Status> {
        println!("got a request: {:?}", request);
        let req: BtcPaymentRequest = request.into_inner();
        let reply = BtcPaymentResponse {
            successful: true,
            message: format!("sent {} btc to {}", req.amount, req.to_address).into(),
        };
        Ok(Response::new(reply))
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let addr = "[::1]:50051".parse()?;
    let btc_service = BitcoinService::default();
    Server::builder()
        .add_service(BitcoinServer::new(btc_service))
        .serve(addr)
        .await?;
    Ok(())
}
