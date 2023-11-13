use std::{time::Duration, borrow::Cow};
use opentelemetry_otlp::{Protocol, new_exporter, new_pipeline, WithExportConfig};
use opentelemetry_sdk::{Resource, runtime, trace};
use opentelemetry_semantic_conventions::resource;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
use opentelemetry::KeyValue;
use crate::CONFIG;

pub struct Tracer { }

impl Tracer {
  // new creates a new tracer and OpenTelemetry trace exporter.
  pub fn new( ) {
    let traceExporter= new_exporter( )
      .tonic( )
      .with_endpoint(&*CONFIG.JAEGER_COLLECTOR_URL)
      .with_protocol(Protocol::Grpc)
      .with_timeout(Duration::from_secs(3));

    let tracer= new_pipeline( )
      .tracing( )
      .with_exporter(traceExporter)
      .with_trace_config(
        trace::Config {
          resource: Cow::Owned(Resource::new(vec!{
            KeyValue::new(resource::SERVICE_NAME, "Profile Microservice")
          })),

          ..Default::default( )
        }
      )
      .install_batch(runtime::Tokio) // Batch export spans.
      .expect("Error creating tracer provider");

    // Spans captured by the 'tracing' crate, will be passed on to the above OpenTelemetry tracer.
    // That tracer will then send the spans to the Jaeger collector.
    tracing_subscriber::registry( )
      .with(tracing_opentelemetry::layer( ).with_tracer(tracer))
      .try_init( )
      .expect("Error initiliazing tracing-subscriber for OpenTelemetry");

    println!("INFO: Created OpenTelemetry tracer and trace exporter successfully");
  }
}