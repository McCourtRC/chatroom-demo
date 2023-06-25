use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloRequest, HelloResponse};

use chatroom::chat_room_server::{ChatRoom, ChatRoomServer};
use chatroom::{
    JoinRequest, JoinResponse, LeaveRequest, LeaveResponse, SendMessageRequest, SendMessageResponse,
};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

pub mod chatroom {
    tonic::include_proto!("chatroom");
}

#[derive(Debug, Default)]
pub struct GreeterService {}

#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloResponse>, Status> {
        let reply = hello_world::HelloResponse {
            message: format!("Hello {}!", request.into_inner().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[derive(Debug, Default)]
pub struct ChatRoomService {}

#[tonic::async_trait]
impl ChatRoom for ChatRoomService {
    async fn join(&self, request: Request<JoinRequest>) -> Result<Response<JoinResponse>, Status> {
        let reply = chatroom::JoinResponse {
            success: true,
            message: format!("{} joined", request.into_inner().username).into(),
        };

        Ok(Response::new(reply))
    }

    async fn leave(
        &self,
        request: Request<LeaveRequest>,
    ) -> Result<Response<LeaveResponse>, Status> {
        let reply = chatroom::LeaveResponse {
            success: true,
            message: format!("{} left", request.into_inner().username).into(),
        };

        Ok(Response::new(reply))
    }

    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        let reply = chatroom::SendMessageResponse {
            message: format!("received message {}", request.into_inner().message).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = GreeterService::default();
    let chatroom = ChatRoomService::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .add_service(ChatRoomServer::new(chatroom))
        .serve(addr)
        .await?;

    Ok(())
}
