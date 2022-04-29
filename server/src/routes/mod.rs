mod register;

use self::register::Register;

use axum::Router;

trait Controller {
    fn controller() -> Router;
}

pub fn routes() -> Router {
    let router = Router::new();

    router.merge(Register::controller())
}
