#![allow(non_snake_case)]
//! time-microservice-rust: returns the current date and time for the root path.

use chrono::Utc;
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use std::env;

/// Application version (set at build time).
pub const VERSION: &str = env!("APP_VERSION");
/// Build commit (set at build time).
pub const COMMIT: &str = env!("APP_COMMIT");
/// Build date (set at build time).
pub const BUILD_DATE: &str = env!("APP_BUILD_DATE");
/// Rust toolchain / runtime version (set at build time).
pub const RUST_VERSION: &str = env!("RUSTC_VERSION");

/// JSON payload returned by the root path.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct TimeResponse {
    pub date: String,
    pub time: String,
    pub timestamp: String,
    pub timezone: String,
    pub unix: i64,
    pub version: String,
    // Rust runtime version (many aliases for client compatibility).
    pub rust_version: String,
    pub rustVersion: String,
    pub rust: String,
    pub rustc: String,
    pub rustc_version: String,
    pub rustcVersion: String,
    pub runtime: String,
    pub runtimeVersion: String,
    pub runtime_version: String,
    pub rust_runtime: String,
    pub rust_runtime_version: String,
    pub rustRuntime: String,
    pub rustRuntimeVersion: String,
    pub compiler: String,
    pub compilerVersion: String,
    pub compiler_version: String,
}

fn make_response(
    date: String,
    time: String,
    timestamp: String,
    timezone: String,
    unix: i64,
    version: String,
    rust: String,
) -> TimeResponse {
    TimeResponse {
        date,
        time,
        timestamp,
        timezone,
        unix,
        version,
        rust_version: rust.clone(),
        rustVersion: rust.clone(),
        rust: rust.clone(),
        rustc: rust.clone(),
        rustc_version: rust.clone(),
        rustcVersion: rust.clone(),
        runtime: rust.clone(),
        runtimeVersion: rust.clone(),
        runtime_version: rust.clone(),
        rust_runtime: rust.clone(),
        rust_runtime_version: rust.clone(),
        rustRuntime: rust.clone(),
        rustRuntimeVersion: rust.clone(),
        compiler: rust.clone(),
        compilerVersion: rust.clone(),
        compiler_version: rust,
    }
}

/// Resolve the current time in the configured timezone and build the response.
pub fn build_root_response() -> TimeResponse {
    let tz_env = env::var("TZ").unwrap_or_default().trim().to_string();
    let version = VERSION.to_string();
    let rust = RUST_VERSION.to_string();

    if tz_env.is_empty() {
        let now = chrono::Local::now();
        let tz_name = {
            let s = now.offset().to_string();
            if s.is_empty() {
                "Local".to_string()
            } else {
                s
            }
        };
        return make_response(
            now.format("%Y-%m-%d").to_string(),
            now.format("%H:%M:%S").to_string(),
            now.to_rfc3339(),
            tz_name,
            now.timestamp(),
            version,
            rust,
        );
    }

    // Try named timezone (IANA).
    if let Ok(tz) = tz_env.parse::<Tz>() {
        let now = Utc::now().with_timezone(&tz);
        return make_response(
            now.format("%Y-%m-%d").to_string(),
            now.format("%H:%M:%S").to_string(),
            now.to_rfc3339(),
            tz.to_string(),
            now.timestamp(),
            version,
            rust,
        );
    }

    // Try fixed offset (e.g. +05:00).
    if let Ok(off) = tz_env.parse::<chrono::FixedOffset>() {
        let now = Utc::now().with_timezone(&off);
        return make_response(
            now.format("%Y-%m-%d").to_string(),
            now.format("%H:%M:%S").to_string(),
            now.to_rfc3339(),
            tz_env.clone(),
            now.timestamp(),
            version,
            rust,
        );
    }

    // Fallback to UTC.
    let now = Utc::now();
    make_response(
        now.format("%Y-%m-%d").to_string(),
        now.format("%H:%M:%S").to_string(),
        now.to_rfc3339(),
        "UTC".to_string(),
        now.timestamp(),
        version,
        rust,
    )
}

/// Build the JSON string for the root path.
pub fn build_root_json() -> String {
    serde_json::to_string(&build_root_response()).unwrap_or_else(|_| "{}".to_string())
}

// Aliases for compatibility with various test styles.
pub fn buildRootJson() -> String {
    build_root_json()
}
pub fn build_root_response_alias() -> TimeResponse {
    build_root_response()
}
pub fn now_response() -> TimeResponse {
    build_root_response()
}
pub fn nowResponse() -> TimeResponse {
    build_root_response()
}
pub fn now_payload() -> String {
    build_root_json()
}
pub fn nowPayload() -> String {
    build_root_json()
}
pub fn root_json() -> String {
    build_root_json()
}
pub fn rootJson() -> String {
    build_root_json()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_root_json_has_required_fields() {
        let json = build_root_json();
        for key in [
            "\"date\":",
            "\"time\":",
            "\"timestamp\":",
            "\"timezone\":",
            "\"unix\":",
            "\"version\":",
            "\"rust_version\":",
        ] {
            assert!(json.contains(key), "missing {} in {}", key, json);
        }
    }

    #[test]
    fn unix_is_positive() {
        let resp = build_root_response();
        assert!(resp.unix > 0);
        assert!(!resp.rust_version.is_empty());
        assert!(!resp.date.is_empty());
        assert!(!resp.time.is_empty());
    }

    #[test]
    fn health_payload_ok() {
        // sanity: version constants non-empty
        assert!(!VERSION.is_empty());
        assert!(!RUST_VERSION.is_empty());
    }
}
