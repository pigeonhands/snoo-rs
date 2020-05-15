use snoo::Reddit;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = Reddit::new()?;

    let spez = r.user("spez").get().await?;
    println!("{}", spez.name());
    println!("Is emplyee: {}", spez.is_employee());
    println!("Has gold: {}", spez.has_gold());
    println!("Verified: {}", spez.is_verified());

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
