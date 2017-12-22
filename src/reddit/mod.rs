pub mod api;
pub mod authenticate;


use serde_json;
use reddit::api::RedditApi;

#[derive(Serialize, Deserialize)]
struct RedditPost {
    subreddit: String,
    selftext_html: Option<String>,
    url: String,
    title: String,
    created_utc: f32,
    id: String
}
#[derive(Serialize, Deserialize)]
struct RedditComment {
    subreddit: String,
    body: String,
    link_title: String,
    link_url: String,
    created_utc: f32,
    id: String
}


pub fn get_posts(api: &mut RedditApi, username: String) -> Vec<serde_json::Value> {
    api.set_api_method(format!("user/{}/submitted", username));

    let response = api.get();

    let posts = response.get("data").unwrap().get("children").unwrap();

    let mut results = Vec::new();

    for i in 0..3 {
        let json = posts[i].get("data").unwrap().clone();

        let post: RedditPost = serde_json::from_value(json).unwrap();

        let result = json!({
            "id": post.id,
            "title": format!("{} in {}: {}", "Post", post.subreddit, post.title),
            "description": post.selftext_html,
            "link": post.url,
            "icon": "NA",
            "timestamp": post.created_utc,
            "feed": "Reddit"
        });
        results.push(result);
    }

    results
}

pub fn get_comments(api: &mut RedditApi, username: String) -> Vec<serde_json::Value> {
    api.set_api_method(format!("user/{}/comments", username));

    let response = api.get();

    let comments = response.get("data").unwrap().get("children").unwrap();

    let mut results = Vec::new();

    for i in 0..3 {
        let json = comments[i].get("data").unwrap().clone();
        let comment: RedditComment = serde_json::from_value(json).unwrap();

        let result = json!({
            "id": comment.id,
            "title": format!("{} in {}: {}", "Comment", comment.subreddit, comment.link_title),
            "description": comment.body,
            "link": comment.link_url,
            "icon": "NA",
            "timestamp": comment.created_utc,
            "feed": "Reddit"
        });

        results.push(result);

    }

    results
}
