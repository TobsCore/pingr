extern crate simple_logger;
extern crate pingr;
use log::{debug, error};
use std::thread;
use log::Level;
use std::env;

fn main() {
    simple_logger::init_with_level(Level::Debug).unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        error!("No base port provided.");
        panic!("No base port provided.");
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

    // Replicas are the amount of web services running. The port numbers go up accordingly from the base
    let replicas = 10;
    let mut handles = vec![];

    // Spin up threads to test all services. Each service is tested by one thread
    for i in 0..replicas {
        let port = port_base + i;
        handles.push(thread::spawn(move || pingr::probe("http", "localhost", port, 1000)));
    }

    // Join all threads after running
    for handle in handles {
        // Wait for the thread to finish. Returns a result.
        let _ = handle.join();
    }
}

