use snoo::Reddit;
use snoo::SearchSort;
use tokio;

//work in progress

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = Reddit::new()?;

    let funny = r.subreddit("all");
    let feed = funny.feed();

    let mut rx = feed.start()?;

    println!("Starting feed...");
    while let Some(d) = rx.recv().await {
        println!("{} \t {}", d.created,  d.name);
    }
    Ok(())
}
