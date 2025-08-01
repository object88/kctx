use async_trait::async_trait;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

#[async_trait]
pub trait Runnable: Send + Sync {
  async fn run(&mut self, cancel_token: CancellationToken);
}

// pub async fn run(cancel_token: CancellationToken, runnables: Vec<Box<dyn Runnable>>) {
pub async fn run(cancel_token: CancellationToken, runnables: impl IntoIterator<Item=Box<dyn Runnable>>) {

  let mut set = JoinSet::new();

  for mut r in runnables {
    let c0 = cancel_token.clone();
    set.spawn(async move {
      r.run(c0).await;
    });
  }

  tokio::select! {
    _ = cancel_token.cancelled() => {

    }
  }

  while let Some(res) = set.join_next().await {
    match res {
      Ok(_val) => println!("Task returned."),
      Err(e) => eprintln!("Task failed: {:?}", e),
    }
  }
}