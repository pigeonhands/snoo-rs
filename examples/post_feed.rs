use snoo::Reddit;
use tokio;
use tokio::time::Duration;

//work in progress

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = Reddit::new()?;

    let all = r.subreddit("all");
    let feed = all.feed()?.delay(Duration::from_secs(5));

    let mut rx = feed.start()?;

    println!("Starting feed...");
    while let Some(d) = rx.recv().await {
        println!("{} \t {}", d.created, d.moderate_data.name);
    }
    Ok(())
}
