fn main() {
    #[cfg(feature = "mysql-bundle")]
    mysqlclient_static();
}

#[cfg(feature = "mysql-bundle")]
fn mysqlclient_static() {
    println!("cargo:rustc-link-search=native=lib");
    println!("cargo:rustc-link-lib=static=mysqlclient");
}