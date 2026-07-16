use velkar_cli_lib::{TerminalOptions, velkar_cli};

#[tokio::main]
async fn main() {
    let result = velkar_cli(TerminalOptions::new().with_prompt("velkar$ "), None).await;
    if let Err(err) = result {
        println!("{err}");
    }
}
