mod handlers;

use self::handlers::create_user;
use super::Controller;

use axum::{routing::post, Router};

pub struct Register;

impl Controller for Register {
    fn controller() -> Router {
        let router = Router::new().route("/", post(create_user));

        Router::new().nest("/register", router)
    }
}
