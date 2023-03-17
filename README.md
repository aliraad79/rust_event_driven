# Event-Driven in Rust
This project try to mock a simple event-driven system in rust. There are three components

## Collector
Which collect datas from clients(publishers) and save it in a redis storage for fast responses

## Data Aggregator
Which collect datas from redis and push them into a postgres db for data consistancy

## Mock publisher
Which issue loads of task to test the system