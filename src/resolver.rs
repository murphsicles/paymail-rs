use crate::errors::PaymailError;
use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};

pub trait Resolver {
    fn resolve_host(&self, domain: &str) -> impl std::future::Future<Output = Result<(String, u16), PaymailError>> + Send;
}

pub struct DefaultResolver;

impl Resolver for DefaultResolver {
    fn resolve_host(&self, domain: &str) -> impl std::future::Future<Output = Result<(String, u16), PaymailError>> + Send {
        async move {
            let resolver =
                TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());
            let srv_query = format!("_bsvalias._tcp.{domain}");
            if let Ok(srv) = resolver.srv_lookup(srv_query).await {
                if let Some(record) = srv.iter().next() {
                    let target = record
                        .target()
                        .to_string()
                        .trim_end_matches('.')
                        .to_string();
                    return Ok((target, record.port()));
                }
            }
            if let Ok(a_lookup) = resolver.ipv4_lookup(domain).await {
                if let Some(ip) = a_lookup.iter().next() {
                    return Ok((ip.to_string(), 443));
                }
            }
            if let Ok(aaaa_lookup) = resolver.ipv6_lookup(domain).await {
                if let Some(ip) = aaaa_lookup.iter().next() {
                    return Ok((ip.to_string(), 443));
                }
            }
            Err(PaymailError::DnsFailure(format!(
                "No host found for {domain}"
            )))
        }
    }
}

pub async fn resolve_host(domain: &str) -> Result<(String, u16), PaymailError> {
    DefaultResolver.resolve_host(domain).await
}
