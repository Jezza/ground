use tracing::Subscriber;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::Layer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

/// Set up a subscriber writing to Stdout.
///
/// `default_env_filter` is used as a fallback if `RUST_LOG` is not set.
/// `emit_json` toggles whether events are formatted in JSON.
pub fn make_stdout_subscriber(
    default_env_filter: impl Into<String>,
    json: bool,
) -> anyhow::Result<impl Subscriber + Sync + Send> {
    make_subscriber(
        default_env_filter,
        json,
        std::io::stdout,
    )
}

/// Set up a subscriber writing to `Sink`.
///
/// `default_env_filter` is used as a fallback if `RUST_LOG` is not set.
/// `emit_json` toggles whether events are formatted in JSON.
pub fn make_subscriber<Sink>(
    default_env_filter: impl Into<String>,
    emit_json: bool,
    sink: Sink,
) -> anyhow::Result<impl Subscriber + Sync + Send>
    where
        Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static + Clone,
{
    let default_env_filter = default_env_filter.into();
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_env_filter));

    let json_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(sink.clone())
        .flatten_event(true)
        .with_current_span(true)
        .with_file(true)
        .with_line_number(true)
        .with_filter(filter_fn(move |_| emit_json));

    let human_readable_layer = tracing_subscriber::fmt::layer()
        .with_writer(sink)
        .with_filter(filter_fn(move |_| !emit_json));

    Ok(Registry::default()
        .with(env_filter)
        .with(json_layer)
        .with(human_readable_layer))
}

/// Set up a subscriber as the global default.
///
/// Returns an error if called more than once.
pub fn set_global_default(subscriber: impl Subscriber + Sync + Send) -> anyhow::Result<()> {
    tracing_log::LogTracer::init()?;
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

