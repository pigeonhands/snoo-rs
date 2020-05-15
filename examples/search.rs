use tokio;
use snoo::Reddit;
use snoo::SearchSort;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = Reddit::new()?;

    let mut pigeon_posts = r.search("pigeon", SearchSort::New).await?;

    for i in 1..4 {
        println!("## Page {}", i);
        for p in pigeon_posts.results() {
            println!("{}", p.info().title)
        }
        if let Some(posts) = pigeon_posts.next().await? {
            pigeon_posts = posts;
        }else{
            println!("No more search results.");
            break;
        }
    }

    Ok(())
}