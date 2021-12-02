//! Internal query builder

use reqwest::StatusCode;
use serde::de::DeserializeOwned;

const API_URL: &str = "https://onlinetools.ups.com/";

// Build the URL for all calls
async fn build<T>(url: String) -> Result<T, StatusCode>
    where
        T: DeserializeOwned,
{
    let response = reqwest::get(url).await;

    match &response {
        Ok(r) => {
            if r.status() != StatusCode::OK {
                return Err(r.status());
            }
        }
        Err(e) => {
            return if e.is_status() {
                Err(e.status().unwrap())
            } else {
                Err(StatusCode::BAD_REQUEST)
            };
        }
    }

    // Parse the response body as Json
    let content = response.unwrap().json::<T>().await;

    match content {
        Ok(s) => Ok(s),
        Err(e) => {
            println!("{:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

// Make call without parameters nor filters
pub async fn call<T>(call: &str) -> Result<T, StatusCode>
    where
        T: DeserializeOwned,
{
    let url = format!("{}/{}", API_URL, call);
    build(url).await
}


#[cfg(test)]
mod tests {
    use reqwest::StatusCode;
    use serde_json::Value;

    use crate::call;

    #[tokio::test]
    async fn error_no_credentials_invalid() {
        let invalid_credential_response: Result<Value, StatusCode> = call("rest/Track").await;
        assert!(invalid_credential_response.is_ok());
    }
}
