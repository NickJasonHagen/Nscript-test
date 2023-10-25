
    pub fn send_message_to_discord_api(msg: &str, api_url: &str) {
        let client = reqwest::blocking::Client::new();
        let body = [("content", msg)];

        match client
            .post(api_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&body)
            .send()
            {
                Ok(_) => println!("Message sent successfully!"),
                Err(err) => eprintln!("Error: {}", err),
            }
    }

