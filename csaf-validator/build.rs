use std::path::Path;

fn main() {
    // Only run these steps if the web feature is enabled
    #[cfg(feature = "web")]
    {
        println!("cargo:rerun-if-changed=../demo/src");
        println!("cargo:rerun-if-changed=../demo/index.html");
        println!("cargo:rerun-if-changed=../csaf-rs/src");
        
        // Check if static files exist, if not, provide helpful error
        let static_dir = Path::new("src/web/static");
        if !static_dir.exists() || std::fs::read_dir(static_dir).unwrap().next().is_none() {
            println!("cargo:warning=Web static files not found. Run 'cargo xtask build-web' to build the frontend.");
        }
    }
}
