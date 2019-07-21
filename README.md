# pingr #

This application connects to multiple web services by calling them via curl and checking the response. This tool was written as a utility tool to test the scalability of microservices written in Rust.

## Why ##

As an introduction to Rust, I wrote a small microservice in Rust and wanted to run these services in a dockerized environment. This environment can be scaled easily by using docker-compose, where you can define replicas of a selected service. To efficiently test these replicas, this tool was written to connect to each service concurrently and simply do some load testing.

## Please note ##

This application is just a fun project to learn rust and play around with it. There is no actual use case to actually using it, I think. 
