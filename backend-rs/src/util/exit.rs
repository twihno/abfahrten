use std::process::exit;

pub fn exit_critical(msg: &str, panic: bool) -> ! {
	if panic {
		panic!("[CRITICAL ERROR] {}", msg);
	}

	print!("[CRITICAL ERROR] {}", msg);
	exit(1);
}
