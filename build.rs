fn main() {
    #[cfg(feature = "mysqlclient-bundle")]
    mysqlclient_static();
}

#[cfg(feature = "mysqlclient-bundle")]
fn mysqlclient_static() {
    println!("cargo:rustc-link-search=native=lib");
    println!("cargo:rustc-link-lib=static=mysqlclient");
}