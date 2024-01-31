// Import necessary libraries
use actix_web::{get, http::StatusCode, web, App, HttpResponse, HttpServer};
use chrono::{DateTime, Local};
use clap::{command, Arg, Command};
use std::fs;
use std::fs::File;
use std::io::Result;

// Define a struct to hold application state
#[derive(Clone)]
struct AppState {
    n_name: String,
    l_name: String,
    f_name: String,
}

// Function to log information to a file
fn log_something(app_state: web::Data<AppState>, log: DateTime<Local>) -> Result<()> {
    // Create a filename based on the 'n_name' field in the app_state
    let path = app_state.n_name.to_owned() + ".txt";

    // Attempt to create the file
    let file_result = File::create(&path);
    if let Err(e) = file_result {
        println!("Error creating file: {:?}", e);
        return Err(e.into());
    }

    let log_str = log.to_string();
    let content =
        app_state.f_name.to_owned() + " created this file to trademark his nickname on " + &log_str;

    // Attempt to write to the file
    let write_result = fs::write(&path, content);
    if let Err(e) = write_result {
        println!("Error writing to file: {:?}", e);
        return Err(e.into());
    }

    Ok(())
}

// Define an Actix-Web handler for the root path (catch-all route)
#[get("**")]
async fn do_log_something(app_state: web::Data<AppState>) -> HttpResponse {
    // Get the current timestamp
    let dt = Local::now();

    // Attempt to log something using the log_something function
    match log_something(app_state.clone(), dt) {
        Ok(..) => HttpResponse::Ok().body("Name Created!!"),
        Err(e) => {
            println!("Error: {:?}", e);
            HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR) // Respond with a 500 Internal Server Error status code
        }
    }
}

// Main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Parse command-line arguments using Clap
    let matches = command!()
        .subcommand(
            Command::new("register-person")
                .arg(
                    Arg::new("firstname")
                        .short('f')
                        .long("first-name, fname, firstname")
                        .required(true)
                        .help("Your first name"),
                )
                .arg(
                    Arg::new("lastname")
                        .short('l')
                        .long("last-name, lname, lastname")
                        .required(true)
                        .help("Your last name"),
                ),
        )
        .arg(
            Arg::new("nickname")
                .short('n')
                .long("nickname, nname, nick-name")
                .required(true)
                .help("The nickname you want to trademark"),
        )
        .get_matches();

    // Retrieve command-line arguments
    let n_name = matches.get_one::<String>("nickname").unwrap();
    let person_args = matches.subcommand_matches("register-person").unwrap();
    let l_name = person_args.get_one::<String>("lastname").unwrap();
    let f_name = person_args.get_one::<String>("firstname").unwrap();

    // Create an instance of the AppState struct
    let app_state = AppState {
        n_name: n_name.to_string(),
        l_name: l_name.to_string(),
        f_name: f_name.to_string(),
    };

    // Start the Actix-Web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone())) // Share app_state across handlers
            .service(web::scope("/").service(do_log_something)) // Register the do_log_something handler under the root path
    })
    .bind(("127.0.0.1", 3002))?
    .run()
    .await
}
