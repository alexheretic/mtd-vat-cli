use crate::auth::Token;
use anyhow::Context;
use std::{io::ErrorKind, path::PathBuf};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

fn cache_dir() -> anyhow::Result<PathBuf> {
    let mut d = dirs::cache_dir().context("no cache dir")?;
    d.push("mtd-vat-cli");
    Ok(d)
}

pub async fn write_token(vrn: &str, token: &Token) -> anyhow::Result<()> {
    let mut token_path = cache_dir()?;
    tokio::fs::create_dir_all(&token_path).await?;
    token_path.push(format!("{vrn}.access.json"));

    let mut file = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(&token_path)
        .await?;

    let data = serde_json::to_vec(token)?;
    file.write_all(&data).await?;
    Ok(())
}

pub async fn read_token(vrn: &str) -> anyhow::Result<Option<Token>> {
    let mut token_path = cache_dir()?;
    token_path.push(format!("{vrn}.access.json"));

    let mut file = match tokio::fs::OpenOptions::new()
        .read(true)
        .open(&token_path)
        .await
    {
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(None),
        x => x?,
    };
    let mut data = Vec::new();
    file.read_to_end(&mut data).await?;

    Ok(Some(serde_json::from_slice(&data)?))
}
