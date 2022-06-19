use clap::Parser;

fn main() {
    async_std::task::Builder::new()
    .name("server".to_string())
    .blocking(server::server("127.0.0.1:7878"));
}

