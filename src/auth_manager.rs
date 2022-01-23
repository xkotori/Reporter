use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct SignupBody {
  pub two_factor_key: String,
  pub hostname: String,
  pub hardware_uuid: String,
}

#[derive(Deserialize)]
pub struct SignupResponse {
  pub access_token: String,
}

#[derive(Deserialize)]
pub struct SignupResponseError {
  pub error: String,
}

#[derive(Debug)]
pub struct AuthManager {}

impl AuthManager {
  /// The signup function that authenticates the machine into Xornet backend.
  pub async fn signup(
    two_factor_key: &str,
    hostname: &str,
    backend_hostname: &str,
    hardware_uuid: &str,
  ) -> Result<SignupResponse> {
    println!("Signing up to Xornet...");

    let client = reqwest::Client::new();
    let response = client
      .post(&format!("https://{backend_hostname}/machines/@signup"))
      .json(&SignupBody {
        two_factor_key: two_factor_key.to_string(),
        hostname: hostname.to_string(),
        hardware_uuid: hardware_uuid.to_string(),
      })
      .send()
      .await?;

    match response.status() {
      reqwest::StatusCode::OK => {
        let response_json: SignupResponse = serde_json::from_str(&response.text().await?)?;
        Ok(response_json)
      }
      reqwest::StatusCode::BAD_REQUEST
      | reqwest::StatusCode::NOT_FOUND
      | reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
        let response_json: SignupResponseError = serde_json::from_str(&response.text().await?)?;
        Err(anyhow::anyhow!(response_json.error))
      }
      _ => Err(anyhow::anyhow!("Unexpected response from Xornet")),
    }
  }
}
