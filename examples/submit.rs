use snoo::{Reddit};
use snoo::items::subreddit::SubredditSubmission;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = Reddit::new_script("snoo-rs", env!("REDDIT_PASSWORD"), env!("REDDIT_ID"), env!("REDDIT_SECRET")).await?;

    let sr = r.subreddit("test");
    let new_post_data = sr.submit("test from snoo-rs", SubredditSubmission::Link("https://github.com/pigeonhands/snoo-rs")).await?;
    println!("{:?}", new_post_data);

    let new_post = r.submission_from_link(&new_post_data.url).await?;
    println!("{:?}", new_post.op().title());
    for comment in new_post.comments() {
        println!("\t{}", comment.body());
    }

    let resp = new_post.op().comment("test comment").await?;
    println!("{:?}", resp);

    Ok(())
}
