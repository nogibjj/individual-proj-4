use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::Value;
use log::{self, LevelFilter};
use simple_logger::SimpleLogger;

async fn function_handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    // Initialize the logger
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    
    // Log the incoming event
    log::info!("Received event: {:?}", event.payload);
    
    // Perform your logic here and generate a response
    Ok(event.payload) // For example, just echo the input for now
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Configure the logger
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    
    // Create the function handler
    let func = service_fn(function_handler);
    
    // Run the lambda function with the handler
    lambda_runtime::run(func).await
}
