use axum::{response::Html, routing::get, Router};

pub fn attach_routes(router: Router) -> Router {
	let router = router.route("/", get(handler));

	router
}

async fn handler() -> Html<&'static str> {
	Html("<h1>Hello, World!</h1>")
}

pub fn create_router() -> Router {
	attach_routes(Router::new())
}
