use poem::{get, post, Route};
use crate::handlers::{get_models, chat_completions};

pub fn create_routes() -> Route {
    Route::new()
        .at("/v1/models", get(get_models))
        .at("/v1/chat/completions", post(chat_completions))
}
