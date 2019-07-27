use curl::easy::Easy;

// Attempts a connection to the given connection. Tries to connect `attempts` times to actually connect
pub fn probe(protocol: &str, host: &str, port: u16, attempts: usize) {
    let mut conn = Easy::new();
    let url_str = format!("{}://{}:{}", protocol, host, port);
    conn.url(&url_str).unwrap();

    for _i in 0..attempts {
        // Establish a connection
        let mut transfer = conn.transfer();
        transfer.write_function(|data| Ok(data.len())).unwrap();
        match transfer.perform() {
            Err(_) => break,
            _ => continue,
        };
    }
}
