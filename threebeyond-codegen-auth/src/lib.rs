#[macro_export]
macro_rules! role {
    ($token:expr, $role:expr, $conn:expr) => {{
        let (conn, allowed) = $token.has_role($role, $conn).await?;

        if !allowed {
            return Err(ApiError::access_missing_role($role));
        }

        conn
    }};
}

#[macro_export]
macro_rules! perm {
    ($token:expr, $perm:expr, $conn:expr) => {{
        let (conn, allowed) = $token.has_perm($perm, $conn).await?;
        let (conn, r) = $token.perms(conn).await?;

        if !allowed {
            return Err(ApiError::access_missing_perm($perm));
        }

        conn
    }};
}
