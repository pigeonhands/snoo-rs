use env_logger;
use snoo::{Reddit, RedditApp, SearchSort};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let app = RedditApp::new_script("snoo-rs", "password", "id", "secret").await?;

    let me = app.me().await?;
    println!("{:?}", me);

    let r = Reddit::from_app(app)?;

    let pigeon_posts = r.search("rust", SearchSort::New).await?;

    let mut search = Some(pigeon_posts);
    while let Some(result) = &search {
        for p in result.results().iter() {
            println!("/r/{}\t\t{}", p.info().subreddit, p.info().title);
        }
        print!("\n");
        search = result.next().await?;
    }
    Ok(())
}
