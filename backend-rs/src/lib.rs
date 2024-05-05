use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod providers;
pub mod util;

pub fn init_tracing() -> () {
	tracing_subscriber::registry()
		.with(
			tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
				// axum logs rejections from built-in extractors with the `axum::rejection`
				// target, at `TRACE` level. `axum::rejection=trace` enables showing those events
				"example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
			}),
		)
		.with(tracing_subscriber::fmt::layer())
		.init();
}
