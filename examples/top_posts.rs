use tokio;
use snoo::Reddit;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = Reddit::new()?;

    let rust_subreddit = r.subreddit("rust");
    let top_posts = rust_subreddit.top().await?;

    for p in top_posts {
        println!("{}", p.info().title)
    }
    Ok(())
}