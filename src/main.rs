use std::process;

fn main() {
    if let Err((code, msg)) = writ::cli::run(std::env::args()) {
        if !msg.is_empty() {
            eprintln!("{}", msg)
        }
        process::exit(code);
    }
}
