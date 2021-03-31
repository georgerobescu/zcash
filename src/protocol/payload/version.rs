use crate::protocol::payload::*;

use chrono::{DateTime, NaiveDateTime, Utc};

use std::{
    io::{self, Cursor, Write},
    net::SocketAddr,
};

#[derive(Debug)]
pub struct Version {
    version: u32,
    services: u64,
    timestamp: DateTime<Utc>,
    addr_recv: (u64, SocketAddr),
    addr_from: (u64, SocketAddr),
    nonce: Nonce,
    user_agent: String,
    start_height: u32,
    relay: bool,
}

impl Version {
    pub fn new(addr_recv: SocketAddr, addr_from: SocketAddr) -> Self {
        Self {
            version: 170_013,
            services: 1,
            timestamp: Utc::now(),
            addr_recv: (1, addr_recv),
            addr_from: (1, addr_from),
            nonce: Nonce::default(),
            user_agent: String::from(""),
            start_height: 0,
            relay: false,
        }
    }

    pub fn encode(&self, buffer: &mut Vec<u8>) -> io::Result<()> {
        buffer.write_all(&self.version.to_le_bytes())?;
        buffer.write_all(&self.services.to_le_bytes())?;
        buffer.write_all(&self.timestamp.timestamp().to_le_bytes())?;

        write_addr(buffer, self.addr_recv)?;
        write_addr(buffer, self.addr_from)?;

        self.nonce.encode(buffer)?;
        write_string(buffer, &self.user_agent)?;
        buffer.write_all(&self.start_height.to_le_bytes())?;
        buffer.write_all(&[self.relay as u8])?;

        Ok(())
    }

    pub fn decode(bytes: &mut Cursor<&[u8]>) -> io::Result<Self> {
        let version = u32::from_le_bytes(read_n_bytes(bytes)?);
        let services = u64::from_le_bytes(read_n_bytes(bytes)?);
        let timestamp = i64::from_le_bytes(read_n_bytes(bytes)?);
        let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);

        let addr_recv = decode_addr(bytes)?;
        let addr_from = decode_addr(bytes)?;

        let nonce = Nonce::decode(bytes)?;
        let user_agent = decode_string(bytes)?;

        let start_height = u32::from_le_bytes(read_n_bytes(bytes)?);
        let relay = u8::from_le_bytes(read_n_bytes(bytes)?) != 0;

        Ok(Self {
            version,
            services,
            timestamp: dt,
            addr_recv,
            addr_from,
            nonce,
            user_agent,
            start_height,
            relay,
        })
    }
}
