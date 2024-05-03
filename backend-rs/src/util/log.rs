use simplelog::{
	ColorChoice, CombinedLogger, ConfigBuilder, LevelFilter, TermLogger, TerminalMode,
};

use crate::util::exit::exit_critical;

pub fn init_logging() {
	let mut raw_logger_config = ConfigBuilder::new();

	let logger_config = match raw_logger_config.set_time_offset_to_local() {
		Ok(local) => local.build(),
		Err(_) => raw_logger_config.build(),
	};

	if let Err(_) = CombinedLogger::init(vec![TermLogger::new(
		LevelFilter::Info,
		logger_config,
		TerminalMode::Mixed,
		ColorChoice::Auto,
	)]) {
		exit_critical("Failed to initialize logging", true);
	}
}
