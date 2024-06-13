
/// grpc_server_max_recv_msg_size: 4194304 (4MB) -- Max gRPC message size that can be received
/// grpc_server_max_send_msg_size: 4194304 (4MB) -- Max gRPC message size that can be sent
#[derive(Clone, Debug)]
#[cfg_attr(feature = "env", derive(ground_env::FromEnv))]
#[cfg_attr(feature = "clap", derive(clap::Parser))]
pub struct OpentelemetryArgs {
    #[cfg_attr(feature = "clap", clap(
        long,
        env,
        default_value = "http://127.0.0.1:4317",
        help = "Otel: Endpoint"
    ))]
    #[cfg_attr(feature = "env", env(default = "http://127.0.0.1:4317"))]
    pub otel_endpoint: String,

    #[cfg_attr(feature = "clap", clap(
        long,
        env,
        default_value = "3s",
        help = "Otel: timeout for the exporter (using humantime::Duration)"
    ))]
    #[cfg_attr(feature = "env", env(default = "3s"))]
    pub otel_timeout: humantime::Duration,

    #[cfg_attr(feature = "clap", clap(
        long,
        env,
        default_value_t = 128,
        help = "Otel: Maximum number of events per span"
    ))]
    #[cfg_attr(feature = "env", env(default = "128"))]
    pub otel_max_events_per_span: u32,

    #[cfg_attr(feature = "clap", clap(
        long,
        env,
        default_value_t = 128,
        help = "Otel: Maximum number of attributes per span"
    ))]
    #[cfg_attr(feature = "env", env(default = "128"))]
    pub otel_max_attributes_per_span: u32,

    #[cfg_attr(feature = "clap", clap(
        long,
        env,
        default_value_t = 128,
        help = "Otel: Maximum number of attributes per event"
    ))]
    #[cfg_attr(feature = "env", env(default = "128"))]
    pub otel_max_attributes_per_event: u32,

    #[cfg_attr(feature = "clap", clap(
        long,
        env,
        default_value_t = 128,
        help = "Otel: Maximum links per span"
    ))]
    #[cfg_attr(feature = "env", env(default = "128"))]
    pub otel_max_links_per_span: u32,

    #[cfg_attr(feature = "clap", clap(
        long,
        env,
        default_value_t = 128,
        help = "Otel: Maximum number of attributes per link"
    ))]
    #[cfg_attr(feature = "env", env(default = "128"))]
    pub otel_max_attributes_per_link: u32,
}

impl Default for OpentelemetryArgs {
    /// Hand-implemented so values do not default to 0, etc
    fn default() -> Self {
        Self {
            otel_endpoint: "http://127.0.0.1:4317".to_string(),
            otel_timeout: std::time::Duration::from_secs(3).into(),
            otel_max_events_per_span: 128,
            otel_max_attributes_per_span: 128,
            otel_max_attributes_per_event: 128,
            otel_max_links_per_span: 128,
            otel_max_attributes_per_link: 128,
        }
    }
}

impl OpentelemetryArgs {
    pub fn to_tracer(&self, service_name: impl Into<String>) -> Result<Option<opentelemetry_sdk::trace::Tracer>, opentelemetry::trace::TraceError> {
        if self.otel_endpoint.is_empty() {
            return Ok(None);
        }

        use opentelemetry::KeyValue;
        use opentelemetry_otlp::WithExportConfig;
        use opentelemetry_sdk::propagation::TraceContextPropagator;
        use opentelemetry_sdk::trace;
        use opentelemetry_sdk::trace::{RandomIdGenerator, Sampler};
        use opentelemetry_sdk::Resource;

        // Set a format for propagating context. This MUST be provided, as the default is a no-op.
        opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

        let exporter = opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(self.otel_endpoint.to_owned())
            .with_timeout(self.otel_timeout.into());

        let trace_config = trace::config()
            .with_sampler(Sampler::ParentBased(Box::new(Sampler::AlwaysOn)))
            .with_id_generator(RandomIdGenerator::default())
            .with_max_events_per_span(self.otel_max_events_per_span)
            .with_max_links_per_span(self.otel_max_links_per_span)
            .with_max_attributes_per_span(self.otel_max_attributes_per_span)
            .with_max_attributes_per_event(self.otel_max_attributes_per_event)
            .with_max_attributes_per_link(self.otel_max_attributes_per_link)
            .with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                service_name.into(),
            )]));

        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(exporter)
            .with_trace_config(trace_config)
            .install_batch(opentelemetry_sdk::runtime::Tokio)?;

        Ok(Some(tracer))
    }
}