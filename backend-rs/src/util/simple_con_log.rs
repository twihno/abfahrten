//! Simple formatted logging to stdout/stderr
use nu_ansi_term::Color::{self, Blue, Red, Yellow};

const MAX_LENGTH_LEVEL_TEXT: usize = 8;

/// Log info to stdout
pub fn con_info(text: &str) {
	print_to_con("Info", Blue, false, text, Stream::Stdout)
}

/// Log warning to stderr
pub fn con_warning(text: &str) {
	print_to_con("WARNING", Yellow, false, text, Stream::Stderr)
}

/// Log error to stderr
pub fn con_error(text: &str) {
	print_to_con("ERROR", Red, false, text, Stream::Stderr)
}

/// Format
pub fn fmt_crit_error_one_line(text: &str) -> String {
	format!("{} {}", Red.bold().underline().paint("[CRITICAL]"), text)
}

pub fn con_crit_error(text: &str) {
	print_to_con("CRITICAL", Red, true, text, Stream::Stderr)
}

enum Stream {
	Stdout,
	Stderr,
}

fn print_to_con(prefix_text: &str, color: Color, underlined: bool, text: &str, stream: Stream) {
	let mut prefix_style = color.bold();

	let current_time = chrono::Utc::now().to_rfc3339();

	if underlined {
		prefix_style = prefix_style.underline();
	}

	let prefix_text_formatted = format!(
		"{} {}{}",
		current_time,
		prefix_style.paint(format!("[{prefix_text}]")),
		str::repeat(" ", MAX_LENGTH_LEVEL_TEXT - prefix_text.len())
	);

	let prefix_empty_line = str::repeat(" ", current_time.len() + MAX_LENGTH_LEVEL_TEXT + 4);

	let mut lines = text.lines();

	let print_method = match stream {
		Stream::Stderr => print_stderr,
		Stream::Stdout => print_stdout,
	};

	let first_line = match lines.next() {
		None => "",
		Some(val) => val,
	};

	print_method(&format!("{prefix_text_formatted} {first_line}"));

	for line in lines {
		print_method(&format!("{prefix_empty_line}{line}"));
	}
}

fn print_stdout(text: &str) {
	println!("{text}");
}

fn print_stderr(text: &str) {
	eprintln!("{text}");
}
