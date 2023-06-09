 #![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
 )]
use reqwest::header::AUTHORIZATION;
use std::env;
use std::collections::HashMap;
use serde_json::Value;

#[tokio::main]
async fn main() {
		tauri::Builder::default()
   	.run(tauri::generate_context!())
   	.expect("error while running tauri application");

	
		let post_endpoint = "https://api.assemblyai.com/v2/transcript";
		
		let mut params = HashMap::new();
		let url = env::var("URL").expect("Couldn't strike url");
		let api_token = env::var("API_TOKEN").expect("Couldn't find api token");
		//println!("url: {}, api token: {}", url, api_token);
		params.insert("audio_url", url);
		params.insert("speaker_labels", "True".to_string());
		
	let client = reqwest::Client::new();
	let res = client.post(post_endpoint)
		.header(AUTHORIZATION, &api_token)
		.json(&params)
		.send()
		.await;
	let res = res.expect("response not found");
	//println!("{:?}", res.status());
	//println!("{:?}", res.headers());
	let input = res.text().await.expect("no text");
	//println!("{}", input);
	let root: Value = serde_json::from_str(&input).expect("unable to parse response");

	// access element using .get()
	let id = root.get("id").expect("id not found").as_str();

	println!("id = {:?}", id); // = Some("a hostname")
	


	//send get request
	let get_endpoint = post_endpoint.to_owned()+"/"+id.expect("id");
	println!("get endpoint: {}", get_endpoint);
	//let result = "";
	loop{
	let res = client.get(&get_endpoint)
		.header(AUTHORIZATION, &api_token)
		.send()
		.await;
	let text = res.expect("response not found").text().await.expect("unable to parse response");
	let root: Value = serde_json::from_str(&text).expect("unable to parse response");

	// access element using .get()
	//let id = root.get("id").expect("id not found").as_str();
	let status = root.get("status").expect("status not found").as_str().expect("status not found");
	if status == "error"{
		println!("{}", text);
		panic!("error");
		break;
	}
	if status == "completed"{
		let result = root.get("text").expect("status not found").as_str().expect("status not found");
		println!("{}",result);
		break;
	}
	
	}

	//make magic happen
	//record audio -> sfml sound recorder buffer thing -> sound buffer
	//store audio as .wav
	//send audio
	fn record_audio(){}

	//tauri front end -> make app happen
	//buttons and stuff
	//amazingness
	fn startup(){}
}

