pub mod cli;
mod pdf;
mod resume;
mod tex;

pub type Error = Result<(), Box<dyn std::error::Error>>;
