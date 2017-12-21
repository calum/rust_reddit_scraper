# Rust Reddit Scraper

A Reddit API client built in Rust. To run the example application run `cargo run`.

## Usage

You need to setup a Reddit Script Application to get credentials for the api. To do so, follow this [guide on Reddit](https://github.com/reddit/reddit/wiki/OAuth2).
```rust
extern crate rust_reddit_user_scraper as scraper;

use scraper::reddit::authenticate::Token;
use scraper::reddit::api::RedditApi;
use scraper::reddit;
use scraper::requester::Requester;

fn main() {
    // setup variables
    ...

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

    // get latest 3 comments from api (can be any user):
    let comments = reddit::get_comments(&mut api, username.clone());

    // get latest 3 posts from api (can be any user):
    let posts = reddit::get_posts(&mut api, username.clone());
}
```

You can make any request to the API by using the `RedditApi` struct. However, you can currently only make `GET` requests to Reddit.
```rust
api.set_api_method(String::from("/api/v1/me/karma"));

let response: serde_json::Value = api.get();
```
The response will be a json object which you can manipulate with the [`serde_json` crate](https://github.com/serde-rs/json).

## Example Application

The example application takes in a few credentials via environment variables and then posts your latest 3 comments and submissions to an API endpoint of your choice.

The application posts look as follows:
```
{api}/feed
{
  "logo": "http://www.redditstatic.com/desktop2x/img/favicon/android-icon-192x192.png",
  "title": "Reddit",
  "description": "Latest comments and posts..."
}

{api}/feed/items
{
  "id": post.id,
  "title": format!("{}: {}", post.subreddit, post.title),
  "description": post.selftext_html,
  "link": post.url,
  "icon": "NA",
  "timestamp": post.created_utc,
  "feed": "Reddit"
}
{
  "id": comment.id,
  "title": format!("{}: {}", comment.subreddit, comment.link_title),
  "description": comment.body,
  "link": comment.link_url,
  "icon": "NA",
  "timestamp": comment.created_utc,
  "feed": "Reddit"
}
```
