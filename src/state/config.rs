use core::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Config {
    listen_addr: SocketAddr,
}

impl Config {
    pub const fn new(listen_addr: SocketAddr) -> Self {
        Self { listen_addr }
    }

    pub const fn listen_addr(&self) -> SocketAddr {
        self.listen_addr
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            listen_addr: SocketAddr::from(([127, 0, 0, 1], 8080)),
        }
    }
}
