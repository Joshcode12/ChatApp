pub fn hash_password(password: &str) -> crate::error::Result<String> {
    if password.len() < 8 {
        return Err(crate::error::AppError::BadRequest(
            "Password length must be at least 8 characters long".to_string(),
        ));
    }

    bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|e| crate::error::AppError::Internal(format!("Password hashing failed: {}", e)))
}

pub fn verify_password(password: &str, hash: &str) -> crate::error::Result<bool> {
    bcrypt::verify(password, hash).map_err(|e| {
        crate::error::AppError::Internal(format!("Password verification failed: {}", e))
    })
}
