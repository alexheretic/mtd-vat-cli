use crate::{API_URL, WWW_URL, error::ResponseExt};
use anyhow::{Context, anyhow};
use axum::{Extension, Router, extract::Query, routing::get};
use std::{
    collections::HashMap,
    future::IntoFuture,
    sync::{Arc, Mutex},
};
use tokio::{net::TcpListener, sync::oneshot};

const REDIRECT_URL: &str = "http://localhost:54786/";

pub async fn user_auth(client_id: &str, client_secret: &str) -> anyhow::Result<Token> {
    let authorize_url = format!(
        "{WWW_URL}/oauth/authorize\
        ?response_type=code\
        &client_id={client_id}\
        &scope=read:vat+write:vat\
        &redirect_uri={REDIRECT_URL}"
    );

    if webbrowser::open(&authorize_url).is_err() {
        eprintln!("Open {authorize_url} in browser to authorize")
    }

    let code = listen_for_redirect().await?;

    let token: Token = reqwest::Client::new()
        .post(format!("{API_URL}/oauth/token"))
        .form(&[
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("grant_type", "authorization_code"),
            ("redirect_uri", REDIRECT_URL),
            ("code", &code),
        ])
        .send()
        .await?
        .error_body_for_status()
        .await?
        .json()
        .await?;

    Ok(token)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Token {
    pub access_token: String,
}

/// Starts an axum server on 54786 to listen for the exchange `code`.
///
/// Returns the first `code` and stops the server.
async fn listen_for_redirect() -> anyhow::Result<String> {
    let (tx, rx) = oneshot::channel();

    let tx = Arc::new(Mutex::new(Some(tx)));

    let server = axum::serve(
        TcpListener::bind("0.0.0.0:54786").await?,
        Router::new()
            .route("/", get(get_redirect))
            .layer(Extension(tx)),
    )
    .into_future();

    tokio::select! {
        r = server => Err(anyhow!("localhost server ended: {r:?}")),
        r = rx => r.context("sender dropped"),
    }
}

async fn get_redirect(
    Extension(tx): Extension<Arc<Mutex<Option<oneshot::Sender<String>>>>>,
    Query(mut params): Query<HashMap<String, String>>,
) -> &'static str {
    if let Some(code) = params.remove("code")
        && let Some(tx) = tx.lock().unwrap().take()
    {
        let _ = tx.send(code);
    }
    "mtd-vat-cli redirect success, continue with CLI"
}
