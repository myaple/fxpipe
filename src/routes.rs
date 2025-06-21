use poem_openapi::OpenApiService;
use poem::{Route, EndpointExt};
use crate::handlers::Api;
use crate::config::AppConfig;

pub fn create_routes(config: AppConfig) -> impl poem::Endpoint {
    let api = Api;
    let api_service = OpenApiService::new(api, "FX Pipe API", "1.0")
        .server("http://localhost:3000/api");
    
    let ui = api_service.clone().swagger_ui();
    Route::new()
        .nest("/api", api_service)
        .nest("/docs", ui)
        .data(config)
}