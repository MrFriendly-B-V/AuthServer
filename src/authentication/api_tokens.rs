pub fn is_api_token_valid(api_token: String) -> bool {
    return unsafe {
        crate::DATABASE.has_api_token(api_token)
    }
}
