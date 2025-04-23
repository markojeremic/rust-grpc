use tonic::{transport::Server, Request, Response, Status};

pub mod model {
    tonic::include_proto!("model");
}

use model::model_service_server::{ModelService, ModelServiceServer};
use model::{ModelRequest, ModelResponse};

#[derive(Default)]
pub struct MyModelService {}

#[tonic::async_trait]
impl ModelService for MyModelService {
    async fn get_model_number(
        &self,
        request: Request<ModelRequest>,
    ) -> Result<Response<ModelResponse>, Status> {
        let model_id = request.into_inner().model_id;
        // random logic to get price
        let model_num = match model_id {
            1 => 100,
            2 => 200,
            _ => 300,
        };
        println!("Received request for model ID: {}", model_id);
        Ok(Response::new(ModelResponse { model_num }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = MyModelService::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(ModelServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
