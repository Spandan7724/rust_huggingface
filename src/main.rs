use std::{
    env,
    io::{self, Write},
};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Serialize)]
struct HFRequest {
    inputs: String,
}

#[derive(Deserialize)]
struct HFResponse {
    generated_text: String,
}

#[derive(Default)]
struct ChatContext {
    chat_history: String,
}

impl ChatContext {
    fn add_to_history(&mut self, user_input: &str, generated_response: &str) {
        self.chat_history.push_str("User Input: ");
        self.chat_history.push_str(user_input.trim());
        self.chat_history.push_str("\nBot Response: ");
        self.chat_history.push_str(generated_response.trim());
        self.chat_history.push_str("\n\n");
    }

    fn save_history(&self, file_path: &str) -> Result<(), std::io::Error> {
        std::fs::write(file_path, &self.chat_history)?;
        Ok(())
    }

    fn clear_history(&mut self) {
        self.chat_history.clear();
    }
}

impl Drop for ChatContext {
    fn drop(&mut self) {
        let history_file = "chat_history.txt";
        if let Err(e) = self.save_history(history_file) {
            eprintln!("Failed to save chat history: {}", e);
        }
        self.clear_history();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let hf_token = env::var("HF_TOKEN").expect("HF_TOKEN environment variable not found");
    let api_url =
        "https://api-inference.huggingface.co/models/mistralai/Mistral-7B-Instruct-v0.2";

    let client = Client::new();
    let mut input = String::new();
    let mut context = ChatContext::default(); 

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let mut user_spec = ColorSpec::new();
    user_spec.set_fg(Some(Color::Cyan)).set_bold(true); 

    let mut bot_spec = ColorSpec::new();
    bot_spec.set_fg(Some(Color::Green)).set_bold(false); 

    let mut error_spec = ColorSpec::new();
    error_spec.set_fg(Some(Color::Red)).set_bold(true); 

    let mut ascii_art_color = ColorSpec::new();
    ascii_art_color.set_fg(Some(Color::Cyan)).set_bold(true); 

    let header = r#"
______ _   _ _____ _____  ______  _____ _____ 
| ___ \ | | /  ___|_   _| | ___ \|  _  |_   _|
| |_/ / | | \ `--.  | |   | |_/ /| | | | | |  
|    /| | | |`--. \ | |   | ___ \| | | | | |  
| |\ \| |_| /\__/ / | |   | |_/ /\ \_/ / | |  
\_| \_|\___/\____/  \_/   \____/  \___/  \_/  
                                                                                                                                                    
"#;

    stdout.set_color(&ascii_art_color)?;
    writeln!(&mut stdout, "{}", header.trim())?;
    stdout.reset()?;
    stdout.flush()?;

    loop {
        print!("> ");
        io::stdout().flush()?;

        io::stdin().read_line(&mut input)?;

        let request_body = HFRequest {
            inputs: format!("{}{}", context.chat_history, input.trim()),
        };

        let response = client
            .post(api_url)
            .header("Authorization", format!("Bearer {}", hf_token))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let response_body: Vec<HFResponse> = response.json().await?;
            let generated_text = &response_body[0].generated_text;

            stdout.set_color(&bot_spec)?;
            writeln!(&mut stdout, "{}", generated_text)?;
            stdout.reset()?;
            stdout.flush()?;

            writeln!(&mut stdout, "------------------------------------------------------------------")?;
            stdout.flush()?;

            context.add_to_history(&input, generated_text);

            input.clear();
        } else {
            writeln!(&mut stdout, "Error: {:?}", response.status())?;
            stdout.set_color(&error_spec)?;
            writeln!(&mut stdout, "Error: {:?}", response.status())?;
            stdout.reset()?;
            stdout.flush()?;
        }
    }
}
