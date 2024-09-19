use base64::{engine::general_purpose, Engine as _};
use dav_server::actix::DavBody;
use http::Request;

pub(crate) fn get_client_ip(req: &Request<DavBody>) -> String {
    if let Some(forwarded_for) = req.headers().get("X-Forwarded-For") {
        if let Ok(forwarded_for_str) = forwarded_for.to_str() {
            return forwarded_for_str
                .split(',')
                .next()
                .unwrap_or("Unknown")
                .to_string();
        } else {
            "Unknown IP".to_string()
        }
    } else {
        "Unknown IP".to_string()
    }
}

pub(crate) fn basic_auth_header(username: &str, password: &str) -> String {
    let credentials = format!("{}:{}", username, password);
    let encoded_credentials = general_purpose::STANDARD.encode(credentials);

    format!("Basic {}", encoded_credentials)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_auth_header_valid() {
        let username = "testuser";
        let password = "testpassword";
        let expected = "Basic dGVzdHVzZXI6dGVzdHBhc3N3b3Jk"; // Base64 encoded "testuser:testpassword"
        let result = basic_auth_header(username, password);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_basic_auth_header_empty_username() {
        let username = "";
        let password = "testpassword";
        let expected = "Basic OnRlc3RwYXNzd29yZA=="; // Base64 encoded ":testpassword"
        let result = basic_auth_header(username, password);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_basic_auth_header_empty_password() {
        let username = "testuser";
        let password = "";
        let expected = "Basic dGVzdHVzZXI6"; // Base64 encoded "testuser:"
        let result = basic_auth_header(username, password);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_basic_auth_header_empty_username_and_password() {
        let username = "";
        let password = "";
        let expected = "Basic Og=="; // Base64 encoded ":"
        let result = basic_auth_header(username, password);
        assert_eq!(result, expected);
    }
}
