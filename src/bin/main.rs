extern crate rust_reddit_user_scraper as scraper;
extern crate dotenv;
#[macro_use]
extern crate serde_json;

use dotenv::dotenv;
use std::env;
use std::process::exit;

use scraper::reddit::authenticate::Token;
use scraper::reddit::api::RedditApi;
use scraper::reddit;
use scraper::requester::Requester;

fn main() {
    // read in the environment variables from a .env file
    dotenv().ok();

    // get the environment variables
    let api_url = env::var("api_url").expect("You must set the api_url environment variable");
    let app_username = env::var("app_username").expect("You must set the app_username environment variable");
    let app_password = env::var("app_secret").expect("You must set the app_secret environment variable");
    let username = env::var("reddit_username").expect("You must set the reddit_username environment variable");
    let password = env::var("reddit_password").expect("You must set the reddit_password environment variable");
    let url = String::from("https://www.reddit.com/api/v1/access_token");

    // get an access token for the reddit api
    let token: Token;
    match Token::new(url, app_username, app_password, username.clone(), password) {
        Ok(access_token) => token = access_token,
        Err(message) => {
            println!("{}", message);
            exit(1);
        }
    }

    // check the token:
    println!("access token: {}", token.get_access_token());

    // create an object to interact with reddit
    let mut api = RedditApi::new(String::from("api/v1/me/karma"), token.get_access_token());

    // get latest 3 comments from api:
    let comments = reddit::get_comments(&mut api, username.clone());

    // get latest 3 posts from api:
    let posts = reddit::get_posts(&mut api, username.clone());

    // post the overall feed meta data to the api
    let feed = json!({
        "logo": "https://www.redditstatic.com/desktop2x/img/favicon/android-icon-192x192.png",
        "title": "Reddit",
        "description": "Latest comments and posts..."
    });
    let endpoint = format!("{}/feed", api_url);
    post_to_api(endpoint, feed.to_string());

    // post the json feed items to the server
    for comment in &comments {
        let json = serde_json::to_string(comment).expect("failed to convert comment to json");

        post_to_api(format!("{}/feed/items", api_url.clone()), json);
    }
    for post in &posts {
        let json = serde_json::to_string(post).expect("failed to convert post to json");

        post_to_api(format!("{}/feed/items", api_url.clone()), json);
    }

    println!("Finished!");
}

fn post_to_api(api_url: String, json: String) {
    let mut http_post_request = Requester::new(api_url);

    // set the body
    http_post_request.set_data(json.clone());

    // add some headers
    http_post_request.add_header("Content-Type", "application/json");
    http_post_request.add_header("User-Agent", "Rust Program");
    http_post_request.add_header("Accept", "application/json");
    http_post_request.add_header("Content-Length", &format!("{}", json.len()));

    // (optional) add the basic authentication
    //http_post_request.set_authorization(app_username, app_password);

    // send the request
    let response = http_post_request.post();

    // print the response to terminal
    println!("Response: {:?}", response);
}
