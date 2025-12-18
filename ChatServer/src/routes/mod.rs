pub fn routes() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .nest("/users", user_routes())
        .nest("/auth", auth_routes())
}

fn user_routes() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route(
            "/deregister/{id}",
            axum::routing::delete(crate::api::user::deregister_user),
        )
        .route(
            "/update",
            axum::routing::post(crate::api::user::update_user),
        )
}

fn auth_routes() -> axum::Router<crate::AppState> {
    axum::Router::new().route(
        "/register",
        axum::routing::post(crate::api::auth::register_user),
    )
}
