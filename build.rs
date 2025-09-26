use std::fmt::Display;

fn main() {
    // Note: プラットフォームによって異なる（Gentooとかは別のslangパッケージとライブラリ名が被ってたりする）ので
    // SLANG_LIB_***を環境変数で指定することでリンク情報を上書きできるようにしている
    rerun_if_env_changed("SLANG_LIB_NAME");
    rerun_if_env_changed("SLANG_LIB_PATH");
    if let Some(libname) = std::env::var_os("SLANG_LIB_NAME") {
        link_lib(&libname.display());
    }
    if let Some(libpath) = std::env::var_os("SLANG_LIB_PATH") {
        link_search(&libpath.display());
    }
}

fn rerun_if_env_changed(env: &(impl Display + ?Sized)) {
    println!("cargo::rerun-if-env-changed={env}");
}

fn link_search(path: &(impl Display + ?Sized)) {
    println!("cargo::rustc-link-search={path}");
}

fn link_lib(name: &(impl Display + ?Sized)) {
    println!("cargo::rustc-link-lib={name}");
}
