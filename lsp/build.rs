fn main() {
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");

    #[cfg(all(target_family = "unix", not(target_os = "macos")))]
    println!("cargo:rustc-link-arg=-Wl,-rpath=$ORIGIN");
}
