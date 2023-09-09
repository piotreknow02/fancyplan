use anyhow::Result;
use reqwest::get;

pub async fn get_plan(url: &str) -> Result<String> {
    let res = get(url)
        .await?
        .text()
        .await?;
    Ok(res)
}
