use std::process::exit;
use tracing::error;

use crate::util::simple_con_log::{con_crit_error, fmt_crit_error_one_line};

/// Terminates the program in the event of a critical error.
///
/// Different behavior based on the value of `panic`:
/// - `panic == true`: panics with the message `[CRITICAL ERROR]: <msg>`
/// - `panic == false`: writes `[CRITICAL ERROR]: <msg>` to stderr and terminates
/// 	with the status code `1`.
pub fn exit_critical(msg: &str, panic: bool) -> ! {
	if panic {
		con_crit_error(msg);
		panic!("{msg}");
	}

	error!(msg);
	con_crit_error(msg);
	exit(1);
}
