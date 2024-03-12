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
use crate::infra::connection::DataBaseConn;
use crate::application::{StaffServiceApi};
use tonic::transport::Server;
use crate::infra::repository::{Repository};
use crate::infra::services::StaffService;
use crate::pb_staff::x_klean_staff_service_server::XKleanStaffServiceServer;

mod adapters;
mod config;
mod application;
mod infra;
mod helpers;

pub mod pb_staff {
    include!("../server/xkleanstaff.rs");
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

    let repo_instance =Repository::new(Arc::clone(&db_conn));
    let repo=Arc::new(repo_instance);


    let staff_service = StaffService::new(Box::new(repo.clone()),Box::new(repo.clone()));


    let service_address = config.get_service_address();
    let api_server = StaffServiceApi::new(Box::new(config),Box::new(staff_service));

    let (signal_tx, signal_rx) = signal_channel();
    spawn(wait_for_sigterm(signal_tx));

    Server::builder().add_service(
        XKleanStaffServiceServer::new(api_server))
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