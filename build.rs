fn main() {
    #[cfg(feature = "mysqlclient-static")]
    mysqlclient_static();
}

#[cfg(feature = "mysqlclient-static")]
fn mysqlclient_static() {
    println!("cargo:rustc-link-search=native=lib");
    println!("cargo:rustc-link-lib=static=mysqlclient");
}