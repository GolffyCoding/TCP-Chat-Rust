use anyhow::Result;
use tokio::net::TcpListener;

pub struct TcpListenerWrapper {
    listener: TcpListener,
    #[allow(dead_code)]
    backlog: usize,
}

impl TcpListenerWrapper {
    pub async fn bind(addr: &str, backlog: usize) -> Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Self { listener, backlog })
    }

    pub async fn accept(&self) -> Result<(tokio::net::TcpStream, std::net::SocketAddr)> {
        let (stream, addr) = self.listener.accept().await?;
        Ok((stream, addr))
    }

    pub fn local_addr(&self) -> Result<std::net::SocketAddr> {
        self.listener.local_addr().map_err(|e| e.into())
    }
}