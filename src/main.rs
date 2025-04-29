use std::process::Command;

use cargo_credential::{Credential, Secret};
use time::{Duration, OffsetDateTime};
use url::Url;

const EXPIRY_SECONDS: usize = 15 * 60;

#[derive(serde::Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: i64,
}

struct JfCredential {}

impl Credential for JfCredential {
    fn perform(
        &self,
        registry: &cargo_credential::RegistryInfo<'_>,
        action: &cargo_credential::Action<'_>,
        _args: &[&str],
    ) -> Result<cargo_credential::CredentialResponse, cargo_credential::Error> {
        match action {
            cargo_credential::Action::Get(_operation) => {}
            _ => {
                return Err(cargo_credential::Error::OperationNotSupported);
            }
        }

        let index_url = Url::parse(&registry.index_url)
            .map_err(|_| cargo_credential::Error::UrlNotSupported)?;
        if !index_url
            .host_str()
            .map(|h| h.ends_with(".jfrog.io"))
            .unwrap_or(false)
        {
            return Err(cargo_credential::Error::UrlNotSupported);
        }

        let now = OffsetDateTime::now_utc();
        let out = Command::new("jf")
            .arg("atc")
            .arg("--expiry")
            .arg(EXPIRY_SECONDS.to_string())
            .output()
            .map_err(|e| {
                cargo_credential::Error::Other(
                    format!("`jf` failed - is it installed? {} ", e).into(),
                )
            })?;

        if !out.status.success() {
            return Err(cargo_credential::Error::Other(
                format!("`jf` failed: {}", String::from_utf8_lossy(&out.stderr)).into(),
            ));
        }

        let TokenResponse {
            access_token,
            expires_in,
        } = serde_json::from_slice(&out.stdout).map_err(|e| {
            cargo_credential::Error::Other(format!("Failed to parse JSON: {}", e).into())
        })?;

        Ok(cargo_credential::CredentialResponse::Get {
            token: Secret::from(format!("Bearer {}", access_token)),
            cache: cargo_credential::CacheControl::Expires {
                expiration: now.saturating_add(Duration::seconds(expires_in)),
            },
            operation_independent: true,
        })
    }
}

fn main() {
    cargo_credential::main(JfCredential {});
}
