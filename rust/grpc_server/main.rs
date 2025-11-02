mod messenger;
use configuration_service_proto::configuration_service::configuration_server::ConfigurationServer;
use group_object_service_proto::group_object_service::group_object_server::GroupObjectServer;
use std::error::Error;
use tonic::transport::Server;

mod services;
mod shutdown_utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "0.0.0.0:5042"
        .parse()
        .expect("[Server]: Failed to parse socket address");

    // Shutdown signal handler
    let signal = shutdown_utils::signal_handler("gRPC Greeter server");

    let dummy = std::sync::Arc::new(services::configuration_service::Dummy {});
    let router = messenger::DirectMessenger {
        configuration_repository: messenger::ConfigurationRepository {},
        dummy: dummy.clone(),
    };

    let config_server =
        ConfigurationServer::new(services::configuration_service::ConfigurationService {
            state: std::sync::Arc::new(router.clone()),
            dummy: dummy.clone(),
        });

    let object_server =
        GroupObjectServer::new(services::group_object_service::GroupObjectService {
            state: std::sync::Arc::new(router.clone()),
        });

    let grpc_server = Server::builder()
        .add_service(config_server)
        .add_service(object_server)
        .serve_with_shutdown(addr, signal);

    let grpc_handle = tokio::spawn(grpc_server);

    println!("ConfigurationServer listening on {}", addr);
    match tokio::try_join!(grpc_handle) {
        Ok(_) => {}
        Err(e) => {
            println!("[Server]: Error: Failed to start gRPC Greeter server.");
            println!("[Server]: Error: {:?}", e);
        }
    }

    Ok(())
}
