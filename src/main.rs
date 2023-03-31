use reqwest::header::AUTHORIZATION;
use std::env;
fn main() {
	  let postEndpoint = "https://api.assemblyai.com/v2/transcript";
		let mut params = HashMap::new();
		let url = env::var("URL").is_ok();
		let api_token = env::var("API_TOKEN").is_ok();
	
		params.insert("audio_url", url);
		params.insert("speaker_labels", "True");
		
    let client = reqwest::Client::new();
		let res = client.post(postEndpoint)
				.header(AUTHORIZATION, api_token)
		    .json(&params)
		    .send()
		    .await?;
		println!("{}", res);
}