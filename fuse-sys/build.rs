fn main() {
    pkg_config::Config::new()
        .atleast_version("2.6.0")
        .probe("fuse")
        .map_err(|e| eprintln!("{}", e))
        .unwrap();
}
