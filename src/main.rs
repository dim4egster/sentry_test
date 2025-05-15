use tracing::{info, Level};
use tracing_subscriber;
use sentry_tracing::EventFilter;
use tracing_subscriber::prelude::*;

fn main() {
    init_tracing();
    tracing::error!("thank you god: error in main");
}

fn init_tracing() {

    let sentry_layer = sentry_tracing::layer().event_filter(|md| match md.level() {
        &tracing::Level::INFO => EventFilter::Event,
        _ => EventFilter::Ignore,
    });

    let fmt_layer = tracing_subscriber::fmt::layer::<tracing_subscriber::Registry>()
        .with_filter(tracing_subscriber::filter::LevelFilter::INFO); // Только INFO и выше

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(sentry_layer)
        .init();

    // Init sentry with tracing feature
    let sentry_dsn = std::env::var("SENTRY_DSN").expect("SENTRY_DSN must be set");
    println!("SENTRY_DSN: {}", sentry_dsn);
    let _guard: sentry::ClientInitGuard = sentry::init((
        sentry_dsn,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            traces_sample_rate: 1.0,
            send_default_pii: true,
            ..Default::default()
        },
    ));
    tracing::info!("thank you god: info 9 in init_tracing");
}