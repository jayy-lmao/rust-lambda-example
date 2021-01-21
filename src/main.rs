use std::error::Error;
use serde_derive::{Deserialize,Serialize};
use simple_error::bail;

use lambda_runtime::{error::HandlerError, lambda, Context};


#[derive(Deserialize)]
struct PersonEvent {
    #[serde(rename="firstName")]
    first_name: String,
}

#[derive(Serialize)]
struct MessageOutput {
    message: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(dbg!(lambda!(handler)))
}

fn handler(p: PersonEvent, _c: Context)->Result<MessageOutput, HandlerError> {
    if p.first_name == "" {
        bail!("Empty first name");
    }
    Ok(MessageOutput {
        message: format!("Hello, {}", p.first_name),
    })
}
