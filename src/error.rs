#[async_trait::async_trait]
pub trait ResponseExt {
    /// Returns an `Err` if response is an error, includes body in error message.
    async fn error_body_for_status(self) -> anyhow::Result<Self>
    where
        Self: Sized;
}
#[async_trait::async_trait]
impl ResponseExt for reqwest::Response {
    async fn error_body_for_status(self) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        if !self.status().is_success() {
            let status = self.status().as_u16();
            let url = self.url().clone();
            let body = self.text().await?;
            anyhow::bail!("{url} failed {status}: {body}");
        }
        Ok(self)
    }
}
