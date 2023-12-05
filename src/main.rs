use mev_bot_clone::run;

#[tokio::main]
async fn main() {
    // 您的异步代码
    dotenv::dotenv().ok();
    run().await;
}