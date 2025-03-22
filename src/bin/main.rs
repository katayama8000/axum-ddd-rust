use main::app::run;

#[tokio::main]
async fn main() {
    run().await.expect("Failed to run the app");
}
