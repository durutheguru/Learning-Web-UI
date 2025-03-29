use calculator::calculator_client::CalculatorClient;
use calculator::SquareRequest;


pub mod calculator {
    tonic::include_proto!("calculator"); // The string specified here must match the proto package name
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CalculatorClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(SquareRequest {
        number: 4,
    });

    let response = client.square(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
