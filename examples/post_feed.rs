use snoo::Reddit;
use snoo::SearchSort;
use tokio;

//work in progress

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = Reddit::new()?;

    let funny = r.subreddit("funny");
    let feed = funny.feed();
    
    let rx = feed.start();

    println!("Starting feed...");
    while let Ok(d) = rx.recv() {
        println!("{}", d.name);
    }

    Ok(())
}
