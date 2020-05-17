use snoo::{Reddit, SearchSort};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = Reddit::new_script("snoo-rs", "password", "id", "secret").await?;

    let me = r.api.me().await?;
    println!("{:?}", me);

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
