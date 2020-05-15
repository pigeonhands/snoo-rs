use tokio;
use snoo::Reddit;
use snoo::SearchSort;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = Reddit::new()?;

    println!("## post search");
    let pigeon_posts = r.search("pigeon", SearchSort::New).await?;
    for p in pigeon_posts.results().iter().take(3) {
        println!("/r/{}\t\t{}", p.info().subreddit, p.info().title);
    }

    println!("## subreddit search");
    let pigeon_subreddits = r.search_subreddits("pigeon", SearchSort::New).await?;
    
    for p in pigeon_subreddits.results().iter().take(3) {
        println!("{}", p.info().url)
    }

    
    println!("## post in subreddit search");
    let subreddit = r.subreddit("pigeon");
    let hands_in_pigeon_subreddits = subreddit.search("hands",  SearchSort::New).await?;
    
    for p in hands_in_pigeon_subreddits.results().iter().take(3) {
        println!("/r/{}\t\t{}", p.subreddit().name(), p.info().title);
    }

    Ok(())
}