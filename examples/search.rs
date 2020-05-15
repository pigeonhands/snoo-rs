use tokio;
use snoo::Reddit;
use snoo::SearchSort;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = Reddit::new()?;

    let pigeon_posts = r.search("pigeon", SearchSort::New).await?;
    println!("## post search");
    for p in pigeon_posts.results().iter().take(3) {
        println!("/r/{}\t\t{}", p.info().subreddit, p.info().title);
    }

    let pigeon_subreddits = r.search_subreddits("pigeon", SearchSort::New).await?;
    println!("## subreddit search");
    for p in pigeon_subreddits.results().iter().take(3) {
        println!("{}", p.info().url)
    }

    
    let subreddit = r.subreddit("pigeon");
    let hands_in_pigeon_subreddits = subreddit.search("hands",  SearchSort::New).await?;
    println!("## post in subreddit search");
    for p in hands_in_pigeon_subreddits.results().iter().take(3) {
        println!("/r/{}\t\t{}", p.name(), p.info().title);
    }

    Ok(())
}