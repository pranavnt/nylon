mod model;
mod fine_tuning;
mod dataset;
mod utils;

pub use model::{Model, GPT4};
pub use fine_tuning::{FineTune, FineTunePrompt};
pub use dataset::Dataset;
pub use utils::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let model = GPT4 {};
        let prompt = "Hello, world!";
        let response = model.generate(prompt, 10, 0.7);
        println!("response: {}", response);
        assert!(response.len() > 0);
    }
}
