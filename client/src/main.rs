use tonic::Request;

pub mod model {
    tonic::include_proto!("model");
}

use model::model_service_client::ModelServiceClient;
use model::ModelRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ModelServiceClient::connect("http://[::1]:50051").await?;

    let request = Request::new(ModelRequest { model_id: 42 });

    let response = client.get_model_number(request).await?;

    println!("RESPONSE: {:?}", response.into_inner());

    Ok(())
}
