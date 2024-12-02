#![allow(dead_code)]

use poem::web::Query;
use poem::{get, handler, Route};
use serde::Deserialize;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::ops::BitXor;

#[derive(Deserialize)]
struct Task1 {
    from: Ipv4Addr,
    key: Ipv4Addr,
}

#[handler]
fn dest(Query(q): Query<Task1>) -> String {
    q.from
        .octets()
        .iter()
        .zip(q.key.octets().iter())
        .map(|(&f, &k)| {
            let (res, _) = f.overflowing_add(k);
            res.to_string()
        })
        .collect::<Vec<_>>()
        .join(".")
}

#[derive(Deserialize)]
struct Task2 {
    from: Ipv4Addr,
    to: Ipv4Addr,
}

#[handler]
fn key(Query(q): Query<Task2>) -> String {
    q.to.octets()
        .iter()
        .zip(q.from.octets().iter())
        .map(|(&f, &k)| {
            let (res, _) = f.overflowing_sub(k);
            res.to_string()
        })
        .collect::<Vec<_>>()
        .join(".")
}

#[derive(Deserialize)]
struct Task3Dest {
    from: Ipv6Addr,
    key: Ipv6Addr,
}

#[handler]
fn v6_dest(Query(q): Query<Task3Dest>) -> String {
    let mut result = [0u8; 16];

    for (i, (&f, &k)) in q
        .from
        .octets()
        .iter()
        .zip(q.key.octets().iter())
        .enumerate()
    {
        result[i] = f.bitxor(k);
    }

    Ipv6Addr::from(result).to_string()
}

#[derive(Deserialize)]
struct Task3Key {
    from: Ipv6Addr,
    to: Ipv6Addr,
}

#[handler]
fn v6_key(Query(q): Query<Task3Key>) -> String {
    let mut result = [0u8; 16];

    for (i, (&t, &f)) in q.to.octets().iter().zip(q.from.octets().iter()).enumerate() {
        result[i] = t.bitxor(f);
    }

    Ipv6Addr::from(result).to_string()
}

pub fn route() -> Route {
    Route::new()
        .at("/dest", get(dest))
        .at("/key", get(key))
        .at("/v6/dest", get(v6_dest))
        .at("/v6/key", get(v6_key))
}
