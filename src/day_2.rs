use poem_openapi::param::Query;
use poem_openapi::payload::PlainText;
use poem_openapi::OpenApi;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::ops::BitXor;

pub struct Api;

#[OpenApi(prefix_path = "/2")]
impl Api {
    #[allow(clippy::unused_async)]
    #[oai(path = "/dest", method = "get")]
    async fn dest(&self, from: Query<Ipv4Addr>, key: Query<Ipv4Addr>) -> PlainText<String> {
        PlainText(
            from.octets()
                .iter()
                .zip(key.octets().iter())
                .map(|(&f, &k)| {
                    let (res, _) = f.overflowing_add(k);
                    res.to_string()
                })
                .collect::<Vec<_>>()
                .join("."),
        )
    }

    #[allow(clippy::unused_async)]
    #[oai(path = "/key", method = "get")]
    async fn key(&self, to: Query<Ipv4Addr>, from: Query<Ipv4Addr>) -> PlainText<String> {
        PlainText(
            to.octets()
                .iter()
                .zip(from.octets().iter())
                .map(|(&f, &k)| {
                    let (res, _) = f.overflowing_sub(k);
                    res.to_string()
                })
                .collect::<Vec<_>>()
                .join("."),
        )
    }

    #[allow(clippy::unused_async)]
    #[oai(path = "/v6/dest", method = "get")]
    async fn v6_dest(&self, from: Query<Ipv6Addr>, key: Query<Ipv6Addr>) -> PlainText<String> {
        let mut result = [0u8; 16];

        for (i, (&f, &k)) in from.octets().iter().zip(key.octets().iter()).enumerate() {
            result[i] = f.bitxor(k);
        }

        PlainText(Ipv6Addr::from(result).to_string())
    }

    #[allow(clippy::unused_async)]
    #[oai(path = "/v6/key", method = "get")]
    async fn v6_key(&self, to: Query<Ipv6Addr>, from: Query<Ipv6Addr>) -> PlainText<String> {
        let mut result = [0u8; 16];

        for (i, (&t, &f)) in to.octets().iter().zip(from.octets().iter()).enumerate() {
            result[i] = t.bitxor(f);
        }

        PlainText(Ipv6Addr::from(result).to_string())
    }
}
