# Snoo

[![](https://img.shields.io/crates/v/snoo?style=for-the-badge)](https://crates.io/crates/snoo)

Work in progress reddit client for rust.

See  [/examples](https://github.com/pigeonhands/snoo-rs/tree/master/examples) for more.

```Rust
use snoo::Reddit;
```


```Rust
let r = Reddit::new()?;

let rust_subreddit = r.subreddit("rust");
let top_posts = rust_subreddit.top().await?;

for p in top_posts {
    println!("{}", p.info().title)
}
```