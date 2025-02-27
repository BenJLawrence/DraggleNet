use tonic::{Request, Response, Status};

use crate::draggle_net::{PingRequest, PingResponse};
use crate::draggle_net::draggle_net_client::DraggleNetClient;

pub mod draggle_net {
    tonic::include_proto!("draggle_net");
}


pub struct Client {

}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut client = DraggleNetClient::connect("http://[::1]:50051").await?;

    //TODO make a function for into_request
    let request = tonic::Request::new(PingRequest { message: "hello from the client".to_string() });

    let response = client.send(request).await?;
    println!("Response: {:?}", response);

    Ok(())
}
