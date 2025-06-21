use crate::handlers::{chat_completions, get_models};
use poem::{get, post, Route};

pub fn create_routes() -> Route {
    Route::new()
        .at("/v1/models", get(get_models))
        .at("/v1/chat/completions", post(chat_completions))
}
