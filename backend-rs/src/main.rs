use abfahrten_backend::util::{config, exit::exit_critical, log, serve};
use rocket::launch;

// #[launch]
// fn rocket() -> _ {
// 	log::init_logging();

// 	print!("{}", "3".parse::<u8>().unwrap());

// 	let config = match config::load_config() {
// 		Err(_) => exit_critical("Couldn't load config", true),
// 		Ok(config) => config,
// 	};

// 	serve::start()
// }

fn main(){
	let date1 = chrono::DateTime<chrono::TimeZone::
}
