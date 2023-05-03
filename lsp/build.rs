use std::env;

fn main() {
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");

    #[cfg(all(target_family = "unix", not(target_os = "macos")))]
    println!("cargo:rustc-link-arg=-Wl,-rpath=$ORIGIN");

    let bqnlsp_path = env::var("BQNLSP_BQN_PATH").unwrap_or_else(|_| "../../BQN/".to_string());
    println!("cargo:rustc-env=BQN_PATH={bqnlsp_path}");
}
