#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::sync::Arc;
use ::config::Config;
use tokio::io::split;
use tokio::spawn;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::{Mutex, oneshot};
use tokio::sync::oneshot::{Receiver, Sender};
use crate::adapters::connection::IConnection;
use crate::config::configuration::Configuration as cfg;
use crate::infrastructure::connection::DataBaseConn;
use crate::application::{StaffServiceApi};
use tonic::transport::Server;
use crate::infrastructure::repository::Repository;
use crate::pb_staff::staff_service_server::StaffServiceServer;

mod adapters;
mod config;
mod application;
mod infrastructure;
mod helpers;


pub mod pb_staff {
    include!("../server/serverstaff.rs");
}

#[tokio::main]
async fn main()  -> Result<(), Box<dyn std::error::Error>> {
    let cfg =Arc::new(cfg::init_config());
    let config = Arc::clone(&cfg);

    let connection_url = cfg.get_connection_url();

    // get connection related url for primary database.
    let conn = DataBaseConn::default();
    let db_conn = match conn.connect(connection_url).await {
        Ok(db) => Arc::new(db),
        Err(err) => panic!("{}", err)
    };


    let repo=Arc::new(Repository::new(db_conn));


    let service_address = config.get_service_address();
    let api_server = StaffServiceApi::new(Box::new(config),repo.clone());

    let (signal_tx, signal_rx) = signal_channel();
    spawn(wait_for_sigterm(signal_tx));

    Server::builder().add_service(
        StaffServiceServer::new(api_server))
        .serve_with_shutdown(service_address.parse()?, async {
            signal_rx.await.ok();
        }
    ).await?;

    Ok(())
}

fn signal_channel() -> (Sender<()>, Receiver<()>) { oneshot::channel() }

async fn wait_for_sigterm(tx: Sender<()>) {
    let _ = signal(SignalKind::terminate())
        .expect("failed to install signal handler")
        .recv().await;

    println!("SIGTERM received : shutting down");

    let _ = tx.send(());
}