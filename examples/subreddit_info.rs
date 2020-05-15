use tokio;
use snoo::Reddit;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = Reddit::new()?;

    let rust_subreddit = r.subreddit("rust").get().await?;
    let info = rust_subreddit.info();

    println!("{}", info.title);
    println!("Subs: {:?}", info.subscribers);
    println!("{:?}", info.created_utc());
    Ok(())
}
