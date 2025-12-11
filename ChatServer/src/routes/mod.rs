pub fn api_routes() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route("/test", axum::routing::get("Routes paths"))
        .nest("/users", user_routes())
}

fn user_routes() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route("/test", axum::routing::get("User links"))
        .route(
            "/register",
            axum::routing::post(crate::api::users::register_user),
        )
        .route(
            "/deregister/{id}",
            axum::routing::delete(crate::api::users::deregister_user),
        )
}
