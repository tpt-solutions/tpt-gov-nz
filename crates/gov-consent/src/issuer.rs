use anyhow::{Context, Result};
use base64::Engine;
use ed25519_dalek::VerifyingKey;

/// Decode an Ed25519 public key from standard or URL-safe base64 (padded or
/// unpadded). The browser wallet emits URL-safe base64, while other clients use
/// the standard alphabet.
pub fn decode_public_key(encoded: &str) -> Result<VerifyingKey> {
    let trimmed = encoded.trim();
    let engines = [
        base64::engine::general_purpose::STANDARD,
        base64::engine::general_purpose::URL_SAFE_NO_PAD,
        base64::engine::general_purpose::URL_SAFE,
    ];
    let bytes = engines
        .iter()
        .find_map(|engine| engine.decode(trimmed).ok())
        .context("issuer public key is not valid base64")?;
    let arr: [u8; 32] = bytes
        .try_into()
        .map_err(|_| anyhow::anyhow!("issuer public key must be 32 bytes"))?;
    VerifyingKey::from_bytes(&arr).context("issuer public key is not a valid Ed25519 key")
}

/// Load the identity server's verification key from the environment.
///
/// Reads `TPT__GOV__ISSUER_PUBLIC_KEY` and fails when it is missing: a
/// department service must never silently accept unverified grants.
pub fn issuer_key_from_env() -> Result<VerifyingKey> {
    let encoded = std::env::var("TPT__GOV__ISSUER_PUBLIC_KEY").context(
        "TPT__GOV__ISSUER_PUBLIC_KEY is required — department services verify \
         consent-grant signatures against it (see .env.example)",
    )?;
    if encoded.trim().is_empty() {
        anyhow::bail!("TPT__GOV__ISSUER_PUBLIC_KEY is empty");
    }
    decode_public_key(&encoded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;

    #[test]
    fn decodes_standard_and_url_safe_base64() {
        let key = SigningKey::generate(&mut rand::rngs::OsRng);
        let verifying = key.verifying_key();
        let bytes = verifying.as_bytes();

        let standard = base64::engine::general_purpose::STANDARD.encode(bytes);
        let url_safe = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes);

        assert_eq!(decode_public_key(&standard).unwrap().as_bytes(), bytes);
        assert_eq!(decode_public_key(&url_safe).unwrap().as_bytes(), bytes);
    }

    #[test]
    fn rejects_bad_keys() {
        assert!(decode_public_key("not base64 !!!").is_err());
        assert!(decode_public_key(&base64::engine::general_purpose::STANDARD.encode([0u8; 31]))
            .is_err());
    }
}
