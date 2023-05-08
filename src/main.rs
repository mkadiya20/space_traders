mod utils;
mod models;
mod api;

use std::{ error::Error, io::{self, Write} };

use tokio;

use api::{ base };
use models::{ base::Base };
use utils::{ client::HttpClient };

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = HttpClient::new();

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        match input {
            "exit" => break,
            "show base" => {
                let base: Base = base::get_base_data(&client).await?;
                println!("{:#?}", base);
            },
            _ => {
                // Do something with the user input
                println!("You entered: {}", input);
            },
        }
    }
    
    Ok(())
}
