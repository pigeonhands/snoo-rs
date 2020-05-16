use snoo::Reddit;
use snoo::SearchSort;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = Reddit::new()?;

    println!("## user search");
    let pigeon_users = r.search_users("pigeon", SearchSort::New).await?;
    for u in pigeon_users.results().iter().take(3) {
        println!("/u/{}", u.name());
    }

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
    let hands_in_pigeon_subreddits = subreddit.search("hands", SearchSort::New).await?;

    for p in hands_in_pigeon_subreddits.results().iter().take(3) {
        println!("/r/{}\t\t{}", p.subreddit().name(), p.info().title);
    }

    Ok(())
}
