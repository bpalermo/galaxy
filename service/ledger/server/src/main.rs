use std::error;

use ledger_lib::ledger::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    println!("starting");
    match run().await {
        Ok(()) => Ok(()),
        Err(e) => panic!("Error: {:?}", e),
    }
}
