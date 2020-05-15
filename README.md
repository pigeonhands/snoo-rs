# Snoo


[![](https://img.shields.io/crates/v/fuzz?style=for-the-badge)](https://crates.io/crates/snoo)

Work in progress reddit client for rust

```Rust
let r = Reddit::new()?;

let rust_subreddit = r.subreddit("rust");
let top_posts = rust_subreddit.top().await?;

for p in top_posts {
    println!("{}", p.info().title)
}
```