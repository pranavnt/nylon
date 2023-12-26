use std::string::String;
use openai_api_rust::*;
use openai_api_rust::chat::*;

pub trait Model {
    fn generate(&self, prompt: &str, max_tokens: i32, temperature: f32) -> String;
}

pub struct GPT4;

impl Model for GPT4 {
    fn generate(&self, prompt: &str, max_tokens: i32, temperature: f32) -> String {
        let auth = Auth::from_env().unwrap();
        let openai = OpenAI::new(auth, "https://api.openai.com/v1/");

        let body: ChatBody = ChatBody {
            model: "gpt-4".to_string(),
            max_tokens: Some(max_tokens),
            temperature: Some(temperature),
            top_p: Some(0_f32),
            n: Some(1),
            stream: Some(false),
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
            messages: vec![Message { role: Role::User, content: prompt.to_string() }],
        };
        let rs = openai.chat_completion_create(&body);
        let choice = rs.unwrap().choices;
        let message = &choice[0].message.as_ref().unwrap();

        println!("{}{}", prompt, message.content);

        message.content.clone()
    }
}