pub mod draggle_net {
    tonic::include_proto!("draggle_net");
}

use tonic::{async_trait, Request, Response, Status};
use tonic::transport::Server;
use draggle_net::*;
use crate::draggle_net::draggle_net_server::{DraggleNet, DraggleNetServer};

pub struct DraggleNetService {

}

#[async_trait]
impl DraggleNet for DraggleNetService {
    async fn send(&self, request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        let message = request.into_inner();

        println!("Got a ping: {:?}", message);

        let reply = PingResponse {
            message: "pong from server".to_string(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "[::1]:50051".parse()?;
    let service = DraggleNetService {};

    println!("Server is listening on {}", addr);

    Server::builder().add_service(DraggleNetServer::new(service)).serve(addr).await?;

    Ok(())
}
