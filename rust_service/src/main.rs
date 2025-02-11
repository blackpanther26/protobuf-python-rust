// src/main.rs
use tonic::{transport::Server, Request, Response, Status};

pub mod message {
    tonic::include_proto!("communication");
}

use message::{
    user_service_server::{UserService, UserServiceServer},
    User, UserRequest, UserResponse,
};

#[derive(Default)]
pub struct UserServiceImpl {}

#[tonic::async_trait]
impl UserService for UserServiceImpl {
    async fn get_user(
        &self,
        request: Request<UserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        // Get the user_id once and store it
        let user_request = request.into_inner();
        let user_id = user_request.user_id;

        println!("Got a request for user_id: {}", user_id);

        if user_id == 1 {
            let user = User {
                id: 1,
                name: "John Doe".to_string(),
                email: "john@example.com".to_string(),
                roles: vec!["user".to_string(), "admin".to_string()],
            };

            Ok(Response::new(UserResponse {
                user: Some(user),
                success: true,
                error_message: String::new(),
            }))
        } else {
            Ok(Response::new(UserResponse {
                user: None,
                success: false,
                error_message: format!("User {} not found", user_id),
            }))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = UserServiceImpl::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
