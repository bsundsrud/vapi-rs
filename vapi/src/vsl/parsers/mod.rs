use std::str::FromStr;

use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::{tag, take_till, take_until, take_while},
    character::complete::{alphanumeric1, digit0, digit1, space0},
    combinator::{all_consuming, complete, map, map_res, opt},
    number::complete::double,
    sequence::{delimited, terminated},
    IResult, Parser,
};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Timestamp {
    #[serde(skip_serializing)]
    pub event: String,
    pub ts: f64,
    pub since_start: f64,
    pub since_last_timestamp: f64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ReqStart {
    pub listener: String,
    pub port: u16,
    pub address: String,
}

fn not_whitespace(input: &str) -> IResult<&str, &str> {
    let (rest, val) = terminated(take_till(char::is_whitespace), space0).parse(input)?;
    Ok((rest, val))
}

fn parse_reqstart(input: &str) -> IResult<&str, ReqStart> {
    let (rest, address) = not_whitespace(input)?;
    let (rest, port) = map_res(not_whitespace, |p| p.parse()).parse(rest)?;
    let (rest, listener) = not_whitespace(rest)?;
    Ok((
        rest,
        ReqStart {
            address: address.to_string(),
            port,
            listener: listener.to_string(),
        },
    ))
}

pub fn reqstart(input: &str) -> Result<ReqStart> {
    let (_, rs) = complete(all_consuming(parse_reqstart))
        .parse(input)
        .map_err(|e| anyhow!("Invalid ReqStart: {}", e))?;
    Ok(rs)
}

fn parse_remote_ip(input: &str) -> IResult<&str, &str> {
    let (rest, ip) = take_while(|c: char| c != ',')(input)?;
    Ok((rest, ip))
}

pub fn remote_ip(input: &str) -> Result<String> {
    let (_, ip) = parse_remote_ip(input).map_err(|e| anyhow!("Invalid remote IP: {}", e))?;
    let ip = if ip.contains('.') {
        if let Some(idx) = ip.find(':') {
            &ip[..idx]
        } else {
            ip
        }
    } else {
        ip
    };
    Ok(ip.to_string())
}

fn parse_timestamp(input: &str) -> IResult<&str, Timestamp> {
    let (rest, event) = alphanumeric1(input)?;
    let (rest, _) = tag(":")(rest)?;
    let (rest, ts) = delimited(space0, double, space0).parse(rest)?;
    let (rest, since_start) = delimited(space0, double, space0).parse(rest)?;
    let (rest, since_last_timestamp) = delimited(space0, double, space0).parse(rest)?;
    Ok((
        rest,
        Timestamp {
            event: event.to_string(),
            ts,
            since_start,
            since_last_timestamp,
        },
    ))
}

pub fn timestamp<'a>(tag: &'a str, value: &str) -> Result<(&'a str, Timestamp)> {
    let (_, ts) = complete(all_consuming(parse_timestamp))
        .parse(value)
        .map_err(|e| anyhow!("Invalid timestamp value: {}", e))?;
    Ok((tag, ts))
}

fn parse_header(input: &str) -> IResult<&str, (String, String)> {
    let (rest, key) = take_until(":")(input)?;
    let (rest, _) = tag(":")(rest)?;
    let (value, _) = space0(rest)?;
    Ok(("", (key.into(), value.into())))
}

pub fn parse<T: FromStr>(input: &str) -> Result<T> {
    let val = input.parse::<T>().map_err(|_| anyhow!("Invalid parse"))?;
    Ok(val)
}

pub fn status(value: &str) -> Result<u16> {
    let status = value
        .parse::<u16>()
        .map_err(|e| anyhow!("Invalid Status Code: {}", e))?;
    Ok(status)
}

pub fn header(value: &str) -> Result<(String, String)> {
    let (_, header) = complete(all_consuming(parse_header))
        .parse(value)
        .map_err(|e| anyhow!("Invalid header value: {}", e))?;
    let (key, value) = header;
    Ok((key, value))
}

pub fn unsigned_delimited(input: &str) -> IResult<&str, u64> {
    map_res(delimited(space0, digit1, space0), |s: &str| {
        s.parse::<u64>()
    })
    .parse(input)
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct RequestAccounting {
    pub header_tx: u64,
    pub body_tx: u64,
    pub total_tx: u64,
    pub header_rx: u64,
    pub body_rx: u64,
    pub total_rx: u64,
}

fn parse_reqacct(input: &str) -> IResult<&str, RequestAccounting> {
    let (rest, header_rx) = unsigned_delimited(input)?;
    let (rest, body_rx) = unsigned_delimited(rest)?;
    let (rest, total_rx) = unsigned_delimited(rest)?;
    let (rest, header_tx) = unsigned_delimited(rest)?;
    let (rest, body_tx) = unsigned_delimited(rest)?;
    let (rest, total_tx) = unsigned_delimited(rest)?;
    Ok((
        rest,
        RequestAccounting {
            header_tx,
            body_tx,
            total_tx,
            header_rx,
            body_rx,
            total_rx,
        },
    ))
}

pub fn req_accounting<'a>(tag: &'a str, value: &str) -> Result<(&'a str, RequestAccounting)> {
    let (_, accounting) = complete(all_consuming(parse_reqacct))
        .parse(value)
        .map_err(|e| anyhow!("Invalid request accounting value: {}", e))?;
    Ok((tag, accounting))
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VarnishLink {
    #[serde(rename = "type")]
    pub ty: String,
    pub vxid: u32,
    pub reason: String,
}

fn parse_link(input: &str) -> IResult<&str, VarnishLink> {
    let (rest, ty) = delimited(space0, alphanumeric1, space0).parse(input)?;
    let (rest, vxid) = map_res(delimited(space0, digit1, space0), |s: &str| {
        s.parse::<u32>()
    })
    .parse(rest)?;
    let (rest, reason) = delimited(space0, alphanumeric1, space0).parse(rest)?;

    Ok((
        rest,
        VarnishLink {
            ty: ty.to_string(),
            vxid,
            reason: reason.to_string(),
        },
    ))
}

pub fn link<'a>(tag: &'a str, value: &str) -> Result<(&'a str, VarnishLink)> {
    let (_, l) = complete(all_consuming(parse_link))
        .parse(value)
        .map_err(|e| anyhow!("Invalid link value: {}", e))?;
    Ok((tag, l))
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VarnishTtl {
    pub source: String,
    pub ttl: i64,
    pub grace: i64,
    pub keep: i64,
    pub reference: i64,
    pub age: Option<i64>,
    pub date: Option<i64>,
    pub expires: Option<i64>,
    pub max_age: Option<i64>,
    pub cacheable: bool,
}

fn signed_number(input: &str) -> IResult<&str, i64> {
    let (rest, negative) = opt(nom::character::complete::char('-')).parse(input)?;
    if negative.is_none() {
        map_res(digit0, |s: &str| s.parse::<i64>()).parse(rest)
    } else {
        let (rest, number) = map_res(digit1, |s: &str| s.parse::<i64>()).parse(rest)?;
        Ok((rest, -number))
    }
}

fn signed_delimited(input: &str) -> IResult<&str, i64> {
    delimited(space0, signed_number, space0).parse(input)
}

fn parse_ttl(input: &str) -> IResult<&str, VarnishTtl> {
    let mut is_cacheable = map(delimited(space0, alphanumeric1, space0), |s: &str| {
        s == "cacheable"
    });
    let (rest, source) = delimited(space0, alphanumeric1, space0).parse(input)?;
    let (rest, ttl) = signed_delimited(rest)?;
    let (rest, grace) = signed_delimited(rest)?;
    let (rest, keep) = signed_delimited(rest)?;
    let (rest, reference) = signed_delimited(rest)?;
    let (rest, age) = opt(signed_delimited).parse(rest)?;
    let (rest, date) = opt(signed_delimited).parse(rest)?;
    let (rest, expires) = opt(signed_delimited).parse(rest)?;
    let (rest, max_age) = opt(signed_delimited).parse(rest)?;
    let (rest, cacheable) = is_cacheable.parse(rest)?;
    Ok((
        rest,
        VarnishTtl {
            source: source.to_string(),
            ttl,
            grace,
            keep,
            reference,
            age,
            date,
            expires,
            max_age,
            cacheable,
        },
    ))
}

pub fn ttl<'a>(tag: &'a str, value: &str) -> Result<(&'a str, VarnishTtl)> {
    let (_, t) = complete(all_consuming(parse_ttl))
        .parse(value)
        .map_err(|e| anyhow!("Invalid ttl value: {}", e))?;
    Ok((tag, t))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remote_ip() {
        let test_cases = vec![
            ("127.0.0.1", "127.0.0.1"),
            ("127.0.0.1:7000", "127.0.0.1"),
            (
                "abcd:0000:1234:1:23:f33:320, 10.0.0.1",
                "abcd:0000:1234:1:23:f33:320",
            ),
            ("127.0.0.1, 10.0.0.5", "127.0.0.1"),
            ("127.0.0.1,10.0.0.5,169.172.0.2", "127.0.0.1"),
        ];
        for (t, e) in test_cases {
            let out = remote_ip(&t).unwrap();
            assert_eq!(out, e);
        }
    }

    #[test]
    fn test_reqstart() {
        let test_val = "127.0.0.1 80 a0";
        let (rest, rs) = parse_reqstart(&test_val).unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            rs,
            ReqStart {
                address: "127.0.0.1".into(),
                port: 80,
                listener: "a0".into(),
            }
        );
    }

    #[test]
    fn test_timestamp() {
        let test_val = "Fetch: 100.0 0.0 1.5";
        let (rest, ts) = parse_timestamp(&test_val).unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            ts,
            Timestamp {
                event: "Fetch".into(),
                ts: 100.0,
                since_start: 0.0,
                since_last_timestamp: 1.5
            }
        );
    }

    #[test]
    fn test_header() {
        let test_val = "X-Foo: Bar Baz";
        let res = parse_header(&test_val);
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            ("", ("X-Foo".to_string(), "Bar Baz".to_string()))
        );
    }

    #[test]
    fn test_signed_number() {
        let input = "-123";
        let res = signed_number(&input);
        assert!(res.is_ok());
        let (rest, num) = res.unwrap();
        assert_eq!(rest, "");
        assert_eq!(num, -123i64);

        let input = "94938";
        let res = signed_number(&input);
        assert!(res.is_ok());
        let (rest, num) = res.unwrap();
        assert_eq!(rest, "");
        assert_eq!(num, 94938i64);

        let input = "12-32";
        let res = signed_number(&input);
        assert!(res.is_ok());
        let (rest, num) = res.unwrap();
        assert_eq!(rest, "-32");
        assert_eq!(num, 12i64);

        let input = "-";
        let res = signed_number(&input);
        assert!(res.is_err());

        let input = "";
        let res = signed_number(&input);
        assert!(res.is_err());
    }

    #[test]
    fn test_ttl_rfc() {
        let input = "RFC 60 10 -1 1312966109 1312966109 1312966109 0 60 cacheable";
        let res = parse_ttl(&input);
        assert!(res.is_ok());
        let (rest, ttl) = res.unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            ttl,
            VarnishTtl {
                source: "RFC".to_string(),
                ttl: 60,
                grace: 10,
                keep: -1,
                reference: 1312966109,
                age: Some(1312966109),
                date: Some(1312966109),
                expires: Some(0),
                max_age: Some(60),
                cacheable: true
            }
        );
    }

    #[test]
    fn test_ttl_vcl() {
        let input = "VCL 120 10 0 1312966111 uncacheable";
        let res = parse_ttl(&input);
        assert!(res.is_ok());
        let (rest, ttl) = res.unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            ttl,
            VarnishTtl {
                source: "VCL".to_string(),
                ttl: 120,
                grace: 10,
                keep: 0,
                reference: 1312966111,
                age: None,
                date: None,
                expires: None,
                max_age: None,
                cacheable: false
            }
        );
    }

    #[test]
    fn test_ttl_hfp() {
        let input = "HFP 2 0 0 1312966113 uncacheable";
        let res = parse_ttl(&input);
        assert!(res.is_ok());
        let (rest, ttl) = res.unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            ttl,
            VarnishTtl {
                source: "HFP".to_string(),
                ttl: 2,
                grace: 0,
                keep: 0,
                reference: 1312966113,
                age: None,
                date: None,
                expires: None,
                max_age: None,
                cacheable: false
            }
        );
    }
}
