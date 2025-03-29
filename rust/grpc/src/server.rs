use tonic::{transport::Server, Request, Response, Status};
use calculator::calculator_server::{Calculator, CalculatorServer};
use calculator::{SquareRequest, SquareResponse};


pub mod calculator {
    tonic::include_proto!("calculator"); // The string specified here must match the proto package name
}


#[derive(Default)]
pub struct CalculatorImpl {}


#[tonic::async_trait]
impl Calculator for CalculatorImpl {

    async fn square(
        &self, request: Request<SquareRequest>,
    ) -> Result<Response<SquareResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        
        let number = request.into_inner().number;
        let response = calculator::SquareResponse {
            result: number * number,
        };

        Ok(Response::new(response))
    }

}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let calculator = CalculatorImpl::default();

    Server::builder()
        .add_service(CalculatorServer::new(calculator))
        .serve(addr)
        .await?;

    println!("CalculatorServer listening on {}", addr);

    Ok(())
}

