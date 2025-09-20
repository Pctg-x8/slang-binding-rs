fn main() {
    println!("cargo::rerun-if-env-changed=SLANG_LIB_FULLPATH");
    if let Some(libpath) = std::env::var_os("SLANG_LIB_FULLPATH") {
        println!("cargo::rustc-link-search={}", libpath.display());
    }
}
