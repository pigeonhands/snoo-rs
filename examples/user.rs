use tokio;
use snoo::Reddit;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = Reddit::new()?;

    let spez = r.user("spez").get().await?;
    let about = spez.info();
    println!("{}", spez.name());
    println!("Is emplyee: {}", about.is_employee);
    println!("Has gold: {}", about.has_gold);
    println!("Verified: {}", about.is_verified);

    println!("\n## Submitted");

    for c in spez.submitted().await?.iter().take(5) {
        println!("{}", c.info().title);
    }

    println!("\n## Comments");

    for c in spez.comments().await?.iter().take(5) {
        println!("{}", c.info().body);
    }

    Ok(())
}