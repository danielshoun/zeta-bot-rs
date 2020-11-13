use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use markov::Chain;
use rand::Rng;
use std::env;
use std::sync::Mutex;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::collections::HashMap;

mod message;
mod reply;

#[get("/")]
async fn status() -> HttpResponse {
	HttpResponse::Ok().body("Bot is currently running.")
}

#[post("/")]
async fn groupme_post(
	req_body: web::Json<message::Message>,
	chain: web::Data<Mutex<Chain<String>>>,
	file: web::Data<Mutex<File>>
) -> HttpResponse {
	if req_body.0.sender_type == "bot" {
		HttpResponse::Ok().finish()
	}
	else {
		if req_body.0.text != "" {
			chain.lock().unwrap().feed_str(&*req_body.0.text);
			file.lock().unwrap().write_all(("\n".to_string() + &*req_body.0.text).as_bytes()).expect("Could not write to file.");
		}

		if rand::thread_rng().gen_range(1, 11) == 10 {
			let reply = reply::Reply {
				bot_id: env::var("GROUPME_BOT_ID").ok().get_or_insert("NONE".to_string()).to_string(),
				text: chain.lock().unwrap().generate_str()
			};
			let mut map = HashMap::new();
			map.insert("bot_id", reply.bot_id);
			map.insert("text", reply.text);
			let client = reqwest::Client::new();
			let _res = client.post("https://api.groupme.com/v3/bots/post")
				.json(&map)
				.send()
				.await;
			HttpResponse::Ok().finish()
		} else {
			HttpResponse::Ok().finish()
		}
	}
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();
    HttpServer::new(|| {
		let mut chain = Chain::new();
		chain.feed_file("source.txt").expect("No source file found.");
		let file = OpenOptions::new().append(true).open("source.txt").expect("No source file found.");
    	App::new()
			.data(Mutex::new(chain))
			.data(Mutex::new(file))
    		.service(status)
    		.service(groupme_post)
    })
    .bind(env::var("LISTEN_IP").unwrap() + ":" + &*env::var("LISTEN_PORT").unwrap())?
    .run()
    .await
}
