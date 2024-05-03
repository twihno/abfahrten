use rocket::{get, routes, Build, Rocket};

pub fn start() -> Rocket<Build> {
	#[get("/")]
	fn index() -> &'static str {
		"Hello World"
	}

	rocket::build().mount("/", routes![index])
}
