fn main() {
    // Note:
    // Gentooとかは別のslangパッケージとライブラリ名が被ってるのでSLANG_LIB_FULLPATHを指定することでサーチパスを上書きする必要がある
    println!("cargo::rerun-if-env-changed=SLANG_LIB_FULLPATH");
    if let Some(libpath) = std::env::var_os("SLANG_LIB_FULLPATH") {
        println!("cargo::rustc-link-search={}", libpath.display());
    }
}
