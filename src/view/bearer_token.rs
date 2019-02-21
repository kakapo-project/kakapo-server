

const BEARER: &'static str = "Bearer ";

pub fn parse_bearer_token(data: String) -> Option<String> {
    let is_bearer = data.starts_with(BEARER);
    if !is_bearer {
        error!("must be a Bearer token");
        None
    } else {
        let (_, token_str) = data.split_at(BEARER.len());

        Some(token_str.to_string())
    }
}

pub fn to_bearer_token(token: String) -> String {
    format!("{}{}", &BEARER, &token)
}