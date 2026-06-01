#[derive(Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub use_ssl: bool,
    pub timeout: u64,
}

impl ServerConfig {
    pub fn connect(&self) {
        println!(
            "Connecting to {}:{} (ssl={}) timeout={}",
            self.host,
            self.port,
            self.use_ssl,
            self.timeout
        );
    }
}

pub struct ServerConfigBuilder {
    host: String,
    port: u16,
    use_ssl: bool,
    timeout: u64,
}

impl ServerConfigBuilder {
    pub fn new() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
            use_ssl: false,
            timeout: 30,
        }
    }

    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn ssl(mut self, value: bool) -> Self {
        self.use_ssl = value;
        self
    }

    pub fn timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn build(self) -> ServerConfig {
        ServerConfig {
            host: self.host,
            port: self.port,
            use_ssl: self.use_ssl,
            timeout: self.timeout,
        }
    }
}