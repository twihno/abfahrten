use nu_ansi_term::Color::{Blue, Red, Yellow};

pub fn con_info(text: &str) {
	println!("{} {}", Blue.bold().paint("[INFO]"), text);
}

pub fn con_warning(text: &str) {
	println!("{} {}", Yellow.bold().paint("[WARNING]"), text);
}

pub fn con_error(text: &str) {
	eprintln!("{} {}", Red.bold().paint("[ERROR]"), text);
}

pub fn fmt_crit_error(text: &str) -> String {
	format!(
		"{} {}",
		Red.bold().underline().paint("[CRITICAL ERROR]"),
		text
	)
}

pub fn con_crit_error(text: &str) {
	eprintln!("{}", fmt_crit_error(text));
}
