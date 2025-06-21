use crate::config::AppConfig;
use crate::handlers::{chat_completions, get_models};
use poem::EndpointExt;
use poem::{get, post, Route}; // Required for .data() method

pub fn create_routes(config: AppConfig) -> Route {
    Route::new()
        .at("/v1/models", get(get_models).data(config.clone()))
        .at("/v1/chat/completions", post(chat_completions).data(config))
}
