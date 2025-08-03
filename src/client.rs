use std::collections::HashMap;
use std::sync::Arc;

use async_mutex::Mutex;
use chrono::prelude::*;
use reqwest::Client;
use secp256k1::SecretKey;
use serde_json::Value;
use tokio::time::{Duration, Instant};

use crate::errors::PaymailError;
use crate::models::{
    Capabilities, P2PPaymentDestinationRequest, P2PPaymentDestinationResponse, P2PTxRequest,
    P2PTxResponse, PaymentDestinationResponse, PaymentRequest, PkiResponse,
};
use crate::resolver::Resolver;
use crate::utils;

#[derive(Clone)]
pub struct PaymailClient {
    http: Arc<Client>,
    cache: Arc<Mutex<HashMap<String, (Capabilities, Instant)>>>,
    cache_ttl: Duration,
    priv_key: SecretKey,
    resolver: Arc<dyn Resolver + Send + Sync>,
}

impl PaymailClient {
    pub fn builder() -> PaymailClientBuilder {
        PaymailClientBuilder::default()
    }

    pub async fn get_base_url(&self, domain: &str) -> Result<String, PaymailError> {
        let (host, port) = self.resolver.resolve_host(domain).await?;
        Ok(format!("http://{host}:{port}"))
    }

    pub async fn get_capabilities(&self, domain: &str) -> Result<Capabilities, PaymailError> {
        let mut cache = self.cache.lock().await;
        if let Some((caps, exp)) = cache.get(domain) {
            if Instant::now() < *exp {
                return Ok(caps.clone());
            }
        }
        let base_url = self.get_base_url(domain).await?;
        let url = format!("{base_url}/.well-known/bsvalias");
        let resp: Capabilities = self.http.get(&url).send().await?.json().await?;
        cache.insert(
            domain.to_string(),
            (resp.clone(), Instant::now() + self.cache_ttl),
        );
        Ok(resp)
    }

    pub async fn get_pubkey(&self, paymail: &str) -> Result<String, PaymailError> {
        let (alias, domain) = parse_paymail(paymail)?;
        let caps = self.get_capabilities(&domain).await?;
        let pki_endpoint = get_template(&caps, "pki", &alias, &domain)?;
        let base_url = self.get_base_url(&domain).await?;
        let pki_url = if pki_endpoint.starts_with('/') {
            format!("{base_url}{pki_endpoint}")
        } else {
            pki_endpoint
        };
        let resp: PkiResponse = self.http.get(&pki_url).send().await?.json().await?;
        Ok(resp.pubkey)
    }

    pub async fn get_payment_destination(
        &self,
        paymail: &str,
        mut req: PaymentRequest,
    ) -> Result<String, PaymailError> {
        let (alias, domain) = parse_paymail(paymail)?;
        let caps = self.get_capabilities(&domain).await?;
        let endpoint = get_template(&caps, "paymentDestination", &alias, &domain)?;
        let base_url = self.get_base_url(&domain).await?;
        let full_endpoint = if endpoint.starts_with('/') {
            format!("{base_url}{endpoint}")
        } else {
            endpoint
        };
        req.dt = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        req.signature = utils::generate_signature(&self.priv_key, &req.signable_message())?;
        let resp: PaymentDestinationResponse = self
            .http
            .post(&full_endpoint)
            .json(&req)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp.output)
    }

    pub async fn get_p2p_payment_destination(
        &self,
        paymail: &str,
        satoshis: u64,
    ) -> Result<P2PPaymentDestinationResponse, PaymailError> {
        let (alias, domain) = parse_paymail(paymail)?;
        let caps = self.get_capabilities(&domain).await?;
        let endpoint = get_template(&caps, "2a40af698840", &alias, &domain)?;
        let base_url = self.get_base_url(&domain).await?;
        let full_endpoint = if endpoint.starts_with('/') {
            format!("{base_url}{endpoint}")
        } else {
            endpoint
        };
        let req = P2PPaymentDestinationRequest { satoshis };
        let resp: P2PPaymentDestinationResponse = self
            .http
            .post(&full_endpoint)
            .json(&req)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    pub async fn send_p2p_tx(
        &self,
        paymail: &str,
        hex: &str,
        metadata: Value,
        reference: &str,
    ) -> Result<P2PTxResponse, PaymailError> {
        let (alias, domain) = parse_paymail(paymail)?;
        let caps = self.get_capabilities(&domain).await?;
        let endpoint = get_template(&caps, "5f1323cddf31", &alias, &domain)?;
        let base_url = self.get_base_url(&domain).await?;
        let full_endpoint = if endpoint.starts_with('/') {
            format!("{base_url}{endpoint}")
        } else {
            endpoint
        };
        let message = format!("{hex}|{reference}");
        let signature = utils::generate_signature(&self.priv_key, &message)?;
        let req = P2PTxRequest {
            hex: hex.to_string(),
            metadata,
            reference: reference.to_string(),
            signature,
        };
        let resp: P2PTxResponse = self
            .http
            .post(&full_endpoint)
            .json(&req)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    pub async fn call_extension(
        &self,
        paymail: &str,
        brfc_id: &str,
        body: Option<Value>,
    ) -> Result<Value, PaymailError> {
        let (alias, domain) = parse_paymail(paymail)?;
        let caps = self.get_capabilities(&domain).await?;
        let endpoint = get_template(&caps, brfc_id, &alias, &domain)?;
        let base_url = self.get_base_url(&domain).await?;
        let full_endpoint = if endpoint.starts_with('/') {
            format!("{base_url}{endpoint}")
        } else {
            endpoint
        };
        let resp = if let Some(b) = body {
            self.http.post(&full_endpoint).json(&b).send().await?
        } else {
            self.http.get(&full_endpoint).send().await?
        };
        let json: Value = resp.json().await?;
        Ok(json)
    }
}

pub struct PaymailClientBuilder {
    cache_ttl: Duration,
    resolver: Option<Arc<dyn Resolver + Send + Sync>>,
}

impl Default for PaymailClientBuilder {
    fn default() -> Self {
        Self {
            cache_ttl: Duration::from_secs(3600),
            resolver: None,
        }
    }
}

impl PaymailClientBuilder {
    pub fn cache_ttl(mut self, ttl: Duration) -> Self {
        self.cache_ttl = ttl;
        self
    }

    pub fn resolver(mut self, resolver: Arc<dyn Resolver + Send + Sync>) -> Self {
        self.resolver = Some(resolver);
        self
    }

    pub fn build(self, priv_key: SecretKey) -> PaymailClient {
        PaymailClient {
            http: Arc::new(Client::new()),
            cache: Arc::new(Mutex::new(HashMap::new())),
            cache_ttl: self.cache_ttl,
            priv_key,
            resolver: self
                .resolver
                .unwrap_or_else(|| Arc::new(crate::resolver::DefaultResolver)),
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

fn get_template(
    caps: &Capabilities,
    key: &str,
    alias: &str,
    domain: &str,
) -> Result<String, PaymailError> {
    if let Some(Value::String(tmpl)) = caps.capabilities.get(key) {
        Ok(tmpl
            .replace("{alias}", alias)
            .replace("{domain.tld}", domain))
    } else {
        Err(PaymailError::CapabilityMissing(key.to_string()))
    }
}
