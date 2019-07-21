use curl::easy::Easy;
use log::{debug, error, info};
use std::thread;
extern crate simple_logger;
use log::Level;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        error!("No base port provided.");
        panic!();
    }

    // This port has to be set directly, currently. This should be substituted by program arguments.
    let port_base = match args[1].parse::<u16>() {
        Ok(port) => port,
        Err(e) => {
            error!("Could not parse {}, must be a valid port", args[1]);
            debug!("{}", e);
            panic!()
        }
    };

    simple_logger::init_with_level(Level::Debug).unwrap();

    // Replicas are the amount of web services running. The port numbers go up accordingly from the base
    let replicas = 10;
    let mut handles = vec![];

    // Spin up threads to test all services. Each service is tested by one thread
    for i in 0..replicas {
        let port = port_base + i;
        handles.push(thread::spawn(move || probe("localhost", port, 1000)));
    }

    // Join all threads after running
    for handle in handles {
        // Wait for the thread to finish. Returns a result.
        let _ = handle.join();
    }
}

// Attempts a connection to the given connection. Tries to connect `attempts` times to actually connect
fn probe(host: &str, port: u16, attempts: usize) {
    let url_str: &str = &format!("{}:{}", host, port);
    info!("Probing {}", url_str);

    let mut conn = Easy::new();
    conn.url(url_str).unwrap();

    for i in 0..attempts {
        if i % 100 == 0 {
            debug!("Attempt {} on {}", i, url_str);
        }

        // Establish a connection
        let mut transfer = conn.transfer();
        transfer.write_function(|data| Ok(data.len())).unwrap();
        match transfer.perform() {
            Ok(e) => e,
            Err(e) => {
                error!("{}", e);
                break;
            }
        }
    }
}
