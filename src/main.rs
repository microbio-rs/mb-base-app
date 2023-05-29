#[tokio::main]
async fn main() {
    mb_base_app::rest::run_server().await;
}
