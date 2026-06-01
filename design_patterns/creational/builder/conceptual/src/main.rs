mod config;

use config::ServerConfigBuilder;

fn main() {
    let config = ServerConfigBuilder::new()
        .host("example.com")
        .port(443)
        .ssl(true)
        .timeout(60)
        .build();

    println!("{:#?}", config);
    println!("Host: {}", config.host);
    println!("Port: {}", config.port);

    config.connect();
}