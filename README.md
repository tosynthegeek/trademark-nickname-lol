
# Actix-Web Server with Logging


This is a simple Actix-Web server written in Rust that logs information to a file when accessed. Additionally, it allows users to register their personal information via command-line arguments.

## Features

- Logging: The server logs information to a file based on the provided nickname when accessed.
- Command-Line Interface (CLI) for User Registration: Users can register their personal information (first name, last name, and nickname) using command-line arguments.
- HTTP Endpoint for Logging
- Timestamped Logging
- State Sharing Between Handlers
- Command-Line Arguments Extraction


## Perequisites
Rust programming language installed. [`Install Rust`](https://www.rust-lang.org/tools/install)

## Tools, Frameworks, and Dependencies

- Actix-Web: [Actix Web](https://crates.io/crates/actix-web) is a powerful, pragmatic, and extremely fast web framework for Rust. It provides the foundation for handling HTTP requests, defining routes, and managing the application's state. I built the server around Actix-Web's components, such as ```App``` and ```HttpServer```.

- Clap: [Clap](https://crates.io/crates/clap) is a simple-to-use, efficient, and full-featured library for parsing command-line arguments in Rust. It simplifies the process of defining and extracting values from the command line. I used Clap to define and parse command-line arguments for registering a person. The server uses Clap to extract values such as first name, last name, and nickname from the command line.

- Chrono: [Chrono](https://crates.io/crates/chrono) date and time library for Rust. In this server, I used it to capture the current timestamp for logging purposes.

- File I/O: [Standard Rust libraries for file input/output](https://doc.rust-lang.org/std/fs/struct.File.html#) are used for creating and writing to files. The server uses ```std::fs::File``` and ```std::fs::write``` to create a file and write log entries to it. The file is named based on the provided nickname (app_state.n_name), and the content includes information about who created the file and when.
### Installation

- Clone project

```bash
git clone https://github.com/tosynthegeek/trademark-username-lol.git
cd trademark-username-lol
```
- Build and run the server
```
cargo run
```
Server will be accessible at [`localhost:3000`](http://127.0.0.1:3000)
## Usage/Examples

### Register a Person
To register a person and trigger the logging functionality, use the following command:
```
cargo run -- register-person -f [FIRST_NAME] -l [LAST_NAME] -n [NICKNAME]
```
Replace [FIRST_NAME], [LAST_NAME], and [NICKNAME] with the desired values.

Example: 
```
cargo run -- -n tosynthegeek register-person -f Tosin -l Ojedapo
```

### Access the Logging Endpoint
Once the server is running, you can access the logging endpoint by visiting [`localhost:3000`](http://127.0.0.1:3000) in your browser

## Contributing

Contributions are welcome! Please feel free to open issues or submit pull requests.


## License

[MIT](https://choosealicense.com/licenses/mit/)

