use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use base64::engine::GeneralPurpose;
use base64::{alphabet, engine, Engine};
use flate2::write::ZlibEncoder;
use hmac::{Hmac, Mac};
use log::error;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

// https://rcos.io/static/internal_docs/hmac/index.html
// https://www.tencentcloud.com/ko/document/product/647/35166

#[derive(Serialize, Deserialize)]
struct JsonParam {
    #[serde(rename = "TLS.ver")]
    version: String,
    #[serde(rename = "TLS.identifier")]
    identifier: String,
    #[serde(rename = "TLS.sdkappid")]
    sdk_app_id: u32,
    #[serde(rename = "TLS.expire")]
    expire: u64,
    #[serde(rename = "TLS.time")]
    time: u64,
    #[serde(rename = "TLS.sig")]
    sig: String,
}

impl JsonParam {
    pub fn new(identifier: String, sdk_app_id: u32, expire: u64, time: u64, sig: String) -> Self {
        Self {
            version: "2.0".to_string(),
            identifier,
            sdk_app_id,
            expire,
            time,
            sig,
        }
    }
}

pub struct TencentSig {
    sdk_app_id: u32,
    secret_key: String,
}

impl TencentSig {
    pub fn new(sdk_app_id: u32, secret_key: String) -> Self {
        Self {
            sdk_app_id,
            secret_key,
        }
    }

    pub fn sdk_app_id(&self) -> u32 {
        self.sdk_app_id
    }

    pub fn generate_user_sig(&self, identifier: String, expire: u64) -> Option<String> {
        let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(now) => now.as_secs(),
            Err(e) => {
                error!("SystemTime::now().duration_since(UNIX_EPOCH) error: {}", e);
                return None;
            }
        };

        let base64_engine =
            engine::GeneralPurpose::new(&alphabet::STANDARD, base64::engine::general_purpose::PAD);

        let base64_sig =
            match self.tencent_hmacsha256(&base64_engine, identifier.clone(), now, expire) {
                Ok(base64_sig) => base64_sig,
                Err(e) => {
                    error!("self.tencent_hmacsha256() error: {}", e);
                    return None;
                }
            };

        let json = JsonParam::new(identifier.clone(), self.sdk_app_id, expire, now, base64_sig);

        let buffer = match serde_json::to_vec(&json) {
            Ok(buffer) => buffer,
            Err(e) => {
                error!("serde_json::to_vec() error: {}", e);
                return None;
            }
        };

        // buffer 압축
        let mut compressor = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        if let Err(e) = compressor.write_all(&buffer) {
            error!("compressor.write_all() error: {}", e);
            return None;
        }

        let buffer = match compressor.finish() {
            Ok(buffer) => buffer,
            Err(e) => {
                error!("compressor.finish() error: {}", e);
                return None;
            }
        };

        let user_sig = base64_engine
            .encode(buffer)
            .replace('+', "*")
            .replace('/', "-")
            .replace('=', "_");

        Some(user_sig)
    }

    fn tencent_hmacsha256(
        &self,
        base64_engine: &GeneralPurpose,
        identifier: String,
        curr_time: u64,
        expire: u64,
    ) -> Result<String, anyhow::Error> {
        let mut raw_content_sig = String::new();
        raw_content_sig.push_str("TLS.identifier:");
        raw_content_sig.push_str(&identifier);
        raw_content_sig.push('\n');
        raw_content_sig.push_str("TLS.sdkappid:");
        raw_content_sig.push_str(&self.sdk_app_id.to_string());
        raw_content_sig.push('\n');
        raw_content_sig.push_str("TLS.time:");
        raw_content_sig.push_str(&curr_time.to_string());
        raw_content_sig.push('\n');
        raw_content_sig.push_str("TLS.expire:");
        raw_content_sig.push_str(&expire.to_string());
        raw_content_sig.push('\n');

        let mut hmac = HmacSha256::new_from_slice(self.secret_key.as_bytes())?;

        hmac.update(raw_content_sig.as_bytes());

        let hash = hmac.finalize().into_bytes();

        Ok(base64_engine.encode(hash))
    }
}
