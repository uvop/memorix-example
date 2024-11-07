mod schema_generated;

use memorix_client_redis::StreamExt;
use schema_generated as mx;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    let mut memorix = mx::Memorix::new().await?;
    let memorix_clone = memorix.clone();

    memorix.task.pass_ball.empty(&mx::System::RUST).await?;
    let mut async_iterator = memorix_clone
        .task
        .pass_ball
        .dequeue(&mx::System::RUST)
        .await?;

    println!("Waiting for ball...");
    loop {
        let ball = async_iterator
            .next()
            .await
            .ok_or("Async Iterator shouldn't end")??;
        let bigger_ball = ball + 1;

        println!("Passing the ball with value {bigger_ball}");
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        memorix
            .task
            .pass_ball
            .enqueue(&mx::System::PYTHON, &bigger_ball)
            .await?;
    }
}
