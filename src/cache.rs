use redis::{Client, Connection, Commands};
use anyhow::{Result, Ok};

const CACHE_LIFETIME: usize = 3600; // seconds

pub struct CachePrivoder {
    connection: Connection,
} 

impl CachePrivoder {
    pub fn from_dsn(dsn: &str) -> Result<Self> {
        let client = Client::open(dsn)?;
        let conn = client.get_connection()?;
        Ok(Self { connection: conn })
    }

    pub fn get(&mut self, url: &str) -> Option<String> {
        let res: Option<String> = self.connection.get(url).unwrap_or(None);
        res
    }

    pub fn set(&mut self, url: &str, content: &str) -> Result<()> {
        let _ = self.connection.set_ex(url, content, CACHE_LIFETIME)?;
        Ok(())
    }
}
