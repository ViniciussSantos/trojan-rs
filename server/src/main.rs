use config::Config;
use if_addrs::get_if_addrs;
use std::{net::IpAddr, sync::Arc};

mod api;
mod config;
mod db;
pub mod entities;
mod error;
mod repository;
mod service;

pub use error::Error;
pub use repository::Repository;
pub use service::Service;

fn get_ip_address(interface_name: &str) -> Result<IpAddr, String> {
    let interface = get_if_addrs()
        .map_err(|err| format!("Error retrieving network interfaces: {}", err))?
        .into_iter()
        .find(|iface| iface.name == interface_name)
        .ok_or_else(|| format!("Interface '{}' not found", interface_name))?;

    Ok(interface.ip())
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), anyhow::Error> {
    std::env::set_var("RUST_LOG", "server=info");
    env_logger::init();

    let config = Config::load()?;

    let db_pool = db::connect(&config.database_url).await?;
    db::migrate(&db_pool).await?;

    let service = Service::new(db_pool);
    let app_state = Arc::new(api::AppState::new(service));

    let routes = api::routes::routes(app_state);

    match get_ip_address("eth1") {
        Ok(ip) => {
            log::info!("starting server on: {}:{}", ip, config.port);

            let (_addr, server) =
                warp::serve(routes).bind_with_graceful_shutdown((ip, config.port), async {
                    tokio::signal::ctrl_c()
                        .await
                        .expect("Failed to listen for CRTL+c");
                    log::info!("Shutting down server");
                });

            server.await;
        }
        Err(err) => {
            log::error!("Error getting ip address: {}", err);
        }
    }

    Ok(())
}
