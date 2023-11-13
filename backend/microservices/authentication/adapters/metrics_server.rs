use autometrics::prometheus_exporter;
use tokio::spawn;
use crate::{THREAD_CANCELLATION_TOKEN, CONFIG};
use axum::{Router, routing::get};

pub struct MetricsServer { }

impl MetricsServer {
  // new creates a Prometheus exporter and starts the HTTP metrics server.
  // (A Prometheus exporter is a tool that collects and exposes metrics from an application.
  // Exporters act as an intermediary between the application and the Prometheus server.)
  pub async fn new( ) {
    prometheus_exporter::init( );

    let threadCancellationToken= THREAD_CANCELLATION_TOKEN.clone( );
    spawn(async move {
      let address= format!("[::]:{}", &*CONFIG.METRICS_SERVER_PORT);
      let address= address.parse( )
                          .expect(&format!("Error parsing metrics server address : {}", address));

      let router= Router::new( )
        .route("/metrics", get(| | async { prometheus_exporter::encode_http_response( ) }));

      println!("INFO: Starting metrics server");

      axum::Server::bind(&address)
        .serve(router.into_make_service( ))
        .with_graceful_shutdown(threadCancellationToken.cancelled( ))
        .await
        .expect("Error starting HTTP metrics server");
    });
  }
}