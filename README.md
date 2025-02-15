# rivals-rs

A crate to wrap the [MarvelRivalsAPI](https://marvelrivalsapi.com/).

## Getting started
Right now, this crate is not available on `crates.io`. Instead you have to use a github dependency.

### Example
```rs
#[tokio::main]
async fn main() -> Result<(), rivals::Error> {
    let client = rivals::Client::builder()
        .api_key(std::env::var("API_KEY").unwrap())
        .build()
        .unwrap();

    let battlepass = client.get_battlepass(1).await?;
    println!("{:#?}", battlepass);

    Ok(())
}
```