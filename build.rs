fn main() {
    // When building in docs.rs, we want to set SQLX_OFFLINE mode to true
    if std::env::var_os("DOCS_RS").is_some() {
        println!("cargo:rustc-env=SQLX_OFFLINE=true");
    }
}

// This will ensure that the SQLX_OFFLINE environment variable is set to true when building the project in docs.rs.
