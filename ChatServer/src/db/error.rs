pub fn users_db_error(
    result: Result<crate::models::User, sqlx::Error>,
) -> crate::error::Result<crate::models::User> {
    match result {
        Ok(user) => Ok(user),
        Err(e) => {
            let config: crate::error::DbErrorConfig = crate::error::DbErrorConfig::new()
                .unique("users_username_key", "Username already taken")
                .foreign_key("Invalid profile file ID")
                .check_violation("Username must be 3-30 characters and bio max 500 characters");

            Err(crate::error::handle_db_error(e, config))
        }
    }
}
