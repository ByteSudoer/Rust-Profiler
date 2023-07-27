# Rust-Profiler

This a web-based application used as a system and a resource monitor. It will provide you wit information about current hardware specifications and live CPU/RAM usage.
> This application uses w WebSocket to send the updates asynchronously so you can host this application on your server and you will get the same results with the same refresh rate from various connections.



## Installation

you can clone the repository and compile the application from source code.
> you need to have rust pre-installed
```bash
$ cargo build --release
```
and the run it 
```
$ ./target/release/rust-profiler
```

### Using Docker

```bash
$ docker build -t rust-profiler .
```
and then run it
```bash
$ docker run --init -p 3000:<Port_host> rust-profiler
```

### Binary Install
you can also install the pre-compiled binary from the release section.
> Sure the latest version as that will assure you have all the features implemented.


## Usage
Once the application is running you can access by opening `127.0.0.1:3000` on your browser.
To visualize the core usage you can run `cargo run --bin single_core` which is a simple benchmark that will maximize the load on a single CPU core.
> I will try to add a multi-core and ram cycles in the future. 

## Documentation
You can generate the documentation for the source code using `cargo` by running this command and the check you browser and the full documentation should be present on a newly created tab.
```bash
$ cargo doc --open
```
