use tokio;
use snoo::Reddit;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = Reddit::new()?;

    let spez = r.user("spez");
    let about = spez.about().await?;
    println!("{}", spez.name());
    println!("Is emplyee: {}", about.is_employee);
    println!("Has gold: {}", about.has_gold);
    println!("Verified: {}", about.is_verified);

    Ok(())
}