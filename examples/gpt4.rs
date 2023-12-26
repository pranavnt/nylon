use nylon::GPT4;
use nylon::Model;

fn main() {
    let model = GPT4 {};
    let prompt = "Hello, world! What's your name?";
    let response = model.generate(prompt, 100, 0.7);
    println!("response: {}", response);
    assert!(response.len() > 0);
}
