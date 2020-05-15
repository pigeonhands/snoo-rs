use snoo::Reddit;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = Reddit::new()?;

    let rust_subreddit = r.subreddit("rust").get().await?;

    println!("{}", rust_subreddit.title());
    println!("Subs: {:?}", rust_subreddit.subscribers());
    println!("{:?}", rust_subreddit.created());
    Ok(())
}
