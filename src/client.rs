use std::collections::HashMap;
use std::sync::Arc;

use async_mutex::Mutex;
use chrono::prelude::*;
use reqwest::Client;
use secp256k1::SecretKey;
use serde_json::Value;
use tokio::time::{Duration, Instant};

use crate::errors::PaymailError;
use crate::models::{Capabilities, PaymentDestinationResponse, PaymentRequest, PkiResponse};
use crate::resolver::resolve_host;
use crate::utils;

#[derive(Clone)]
pub struct PaymailClient {
    http: Arc<Client>,
    cache: Arc<Mutex<HashMap<String, (Capabilities, Instant)>>>,
    cache_ttl: Duration,
    priv_key: SecretKey,
}

impl PaymailClient {
    pub fn builder() -> PaymailClientBuilder {
        PaymailClientBuilder::default()
    }

    pub async fn get_capabilities(&self, domain: &str) -> Result<Capabilities, PaymailError> {
        let mut cache = self.cache.lock().await;
        if let Some((caps, exp)) = cache.get(domain) {
            if Instant::now() < *exp {
                return Ok(caps.clone());
            }
        }
        let (host, port) = resolve_host(domain).await?;
        let url = format!("https://{}:{}/.well-known/bsvalias", host, port);
        let resp: Capabilities = self.http.get(&url).send().await?.json().await?;
        cache.insert(domain.to_string(), (resp.clone(), Instant::now() + self.cache_ttl));
        Ok(resp)
    }

    pub async fn get_pubkey(&self, paymail: &str) -> Result<String, PaymailError> {
        let (alias, domain) = parse_paymail(paymail)?;
        let caps = self.get_capabilities(&domain).await?;
        let pki_endpoint = get_template(&caps, "pki", &alias, &domain)?;
        let resp: PkiResponse = self.http.get(&pki_endpoint).send().await?.json().await?;
        Ok(resp.pubkey)
    }

    pub async fn get_payment_destination(&self, paymail: &str, mut req: PaymentRequest) -> Result<String, PaymailError> {
        let (alias, domain) = parse_paymail(paymail)?;
        let caps = self.get_capabilities(&domain).await?;
        let endpoint = get_template(&caps, "paymentDestination", &alias, &domain)?;
        req.dt = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        req.signature = utils::generate_signature(&self.priv_key, &req.signable_message())?;
        let resp: PaymentDestinationResponse = self.http.post(&endpoint).json(&req).send().await?.json().await?;
        Ok(resp.output)
    }

    // TODO: Add methods for P2P payments, tx submission, etc.
}

pub struct PaymailClientBuilder {
    cache_ttl: Duration,
}

impl Default for PaymailClientBuilder {
    fn default() -> Self {
        Self { cache_ttl: Duration::from_secs(3600) }
    }
}

impl PaymailClientBuilder {
    pub fn cache_ttl(mut self, ttl: Duration) -> Self {
        self.cache_ttl = ttl;
        self
    }

    pub fn build(self, priv_key: SecretKey) -> PaymailClient {
        PaymailClient {
            http: Arc::new(Client::new()),
            cache: Arc::new(Mutex::new(HashMap::new())),
            cache_ttl: self.cache_ttl,
            priv_key,
        }
    }
}

fn parse_paymail(paymail: &str) -> Result<(String, String), PaymailError> {
    let parts: Vec<&str> = paymail.split('@').collect();
    if parts.len() != 2 {
        return Err(PaymailError::InvalidFormat(paymail.to_string()));
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

fn get_template(caps: &Capabilities, key: &str, alias: &str, domain: &str) -> Result<String, PaymailError> {
    if let Some(Value::String(tmpl)) = caps.capabilities.get(key) {
        Ok(tmpl.replace("{alias}", alias).replace("{domain.tld}", domain))
    } else {
        Err(PaymailError::CapabilityMissing(key.to_string()))
    }
}
