#[derive(Debug)]
#[cfg_attr(feature = "env", derive(ground_env::FromEnv))]
#[cfg_attr(feature = "clap", derive(clap::Parser))]
#[cfg_attr(feature = "clap", clap(next_help_heading = "TRACING", term_width = 200))]
pub struct TracingArgs {
    #[cfg_attr(feature = "env", env(rename = "HUMAN_READABLE_LOGS", default))]
    #[cfg_attr(feature = "clap", clap(name = "human-readable-logs", env = "HUMAN_READABLE_LOGS", long))]
    pub human_readable_logs: bool,

    #[cfg_attr(feature = "clap", clap(
        long,
        env,
        help = "Configures the default log level filter for this service, ignored when RUST_LOG is set."
    ))]
    pub default_env_filter: Option<String>,

    #[cfg(feature = "cli-otel")]
    #[cfg_attr(feature = "clap", clap(flatten))]
    #[cfg_attr(feature = "env", env(flatten))]
    pub otel: Option<crate::cli::otel::OpentelemetryArgs>,
}

impl TracingArgs {
    #[track_caller]
    pub fn init(&self) -> anyhow::Result<()> {
        self.init_with_sink(std::io::stdout)
    }

    // service_name: &str, service_name: &str
    #[track_caller]
    pub fn init_with_sink<Sink>(&self, sink: Sink) -> anyhow::Result<()>
        where
            Sink: for<'a> tracing_subscriber::fmt::MakeWriter<'a> + Send + Sync + 'static + Clone,
    {
        let json = !self.human_readable_logs;
        let default_env_filter = self.default_env_filter.as_deref().unwrap_or("INFO");
        let subscriber = crate::tracing::make_subscriber(
            default_env_filter,
            json,
            sink,
        )?;

        #[cfg(feature = "cli-otel")]
        {
            let service_name = std::panic::Location::caller().file();
            let tracer = self.otel.as_ref().and_then(|args| args.to_tracer(service_name).transpose());
            if let Some(tracer) = tracer {
                let layer = tracing_opentelemetry::layer().with_tracer(tracer?);
                use tracing_subscriber::layer::SubscriberExt;
                return crate::tracing::set_global_default(subscriber.with(layer));
            }
        }

        crate::tracing::set_global_default(subscriber)
    }
}
