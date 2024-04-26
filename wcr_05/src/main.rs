fn main() {
    if let Err(err) = wcr::run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
