use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use super::base_config::BaseConfig;

pub fn init_tracing_console(log_level: tracing::Level) {
	let subscriber = tracing_subscriber::registry();
	// .with(
	// 	tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
	// 		// axum logs rejections from built-in extractors with the `axum::rejection`
	// 		// target, at `TRACE` level. `axum::rejection=trace` enables showing those events
	// 		"example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
	// 	}),
	// )
	let subscriber = subscriber.with(tracing_subscriber::fmt::layer());

	subscriber.init();
}

pub fn init_tracing(config: &BaseConfig) {
	match config.log_type {
		super::base_config::LogType::NONE => return,
		super::base_config::LogType::CONSOLE => init_tracing_console(config.log_level),
		super::base_config::LogType::FILE(_) => todo!(),
	};
}
