use payments::bitcoin_client::BitcoinClient;
use payments::BtcPaymentRequest;
use tonic::{client, Response};

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BitcoinClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(BtcPaymentRequest {
        from_address: "nithin.n".to_owned(),
        to_address: "rachan".to_owned(),
        amount: 100,
    });
    let res = client.send_payments(request).await;
    match res {
        Ok(res) => {
            let res = res.into_inner();
            println!("client received {} {}", res.message, res.message)
        }
        Err(err) => (),
    }
    Ok(())
}
