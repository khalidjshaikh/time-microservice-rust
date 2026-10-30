use std::env;
use std::process::Command;

fn git_output(args: &[&str]) -> Option<String> {
    let out = Command::new("git").args(args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

fn main() {
    // Rust toolchain / runtime version.
    let rustc_version = Command::new("rustc")
        .arg("--version")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "rustc unknown".to_string());
    println!("cargo:rustc-env=RUSTC_VERSION={}", rustc_version);

    // Application version metadata (env overrides, git fallback).
    let version = env::var("VERSION")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .or_else(|| git_output(&["describe", "--tags", "--always", "--dirty"]))
        .unwrap_or_else(|| "dev".to_string());

    let commit = env::var("COMMIT")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .or_else(|| git_output(&["rev-parse", "--short", "HEAD"]))
        .unwrap_or_else(|| "none".to_string());

    let build_date = env::var("BUILD_DATE")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| {
            Command::new("date")
                .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_else(|_| "unknown".to_string())
        });

    println!("cargo:rustc-env=APP_VERSION={}", version);
    println!("cargo:rustc-env=APP_COMMIT={}", commit);
    println!("cargo:rustc-env=APP_BUILD_DATE={}", build_date);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=VERSION");
    println!("cargo:rerun-if-env-changed=COMMIT");
    println!("cargo:rerun-if-env-changed=BUILD_DATE");
}
