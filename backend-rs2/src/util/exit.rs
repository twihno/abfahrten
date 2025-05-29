//! Methods for terminating the application

use std::process::exit;
use tracing::error;

/// Terminates the program in the event of a critical error.
///
/// Different behavior based on the value of `panic`:
/// - `panic == true`: panics with the message `[CRITICAL ERROR]: <msg>`
/// - `panic == false`: writes `[CRITICAL ERROR]: <msg>` to stderr and terminates
///   with the status code `1`.
pub fn exit_critical(msg: &str, panic: bool) -> ! {
    error!(msg);

    if panic {
        panic!("{msg}");
    }
    exit(1);
}
