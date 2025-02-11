// src/client.rs
use tonic::Request;

pub mod message {
    tonic::include_proto!("communication");
}

use message::user_service_client::UserServiceClient;
use message::UserRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UserServiceClient::connect("http://[::1]:50051").await?;

    // Test with existing user
    let request = Request::new(UserRequest { user_id: 1 });
    match client.get_user(request).await {
        Ok(response) => {
            let response = response.into_inner();
            if response.success {
                let user = response.user.unwrap();
                println!("Found user: {} (ID: {})", user.name, user.id);
                println!("Email: {}", user.email);
                println!("Roles: {}", user.roles.join(", "));
            } else {
                println!("Error: {}", response.error_message);
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    // Test with non-existing user
    let request = Request::new(UserRequest { user_id: 2 });
    match client.get_user(request).await {
        Ok(response) => {
            let response = response.into_inner();
            if !response.success {
                println!("Error: {}", response.error_message);
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}
