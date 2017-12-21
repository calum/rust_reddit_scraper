
use requester::Requester;
use serde_json;

pub struct RedditApi {
    api_method: String,
    access_token: String,
    url: String,
    request: Requester
}

impl RedditApi {
    /// create a new reddit api request
    pub fn new(api_method: String, access_token: String) -> RedditApi {
        let url = format!("https://oauth.reddit.com/{}", api_method.clone());
        RedditApi {
            api_method: api_method,
            access_token: access_token,
            url: url.clone(),
            request: Requester::new(url)
        }
    }

    // set the api_method to a new string
    pub fn set_api_method(&mut self, api_method: String) {
        let new_url = format!("https://oauth.reddit.com/{}", api_method.clone());
        self.api_method = api_method;
        self.url = new_url.clone();
        self.request.set_url(new_url);
    }

    /// update the access_token
    pub fn update_access_token(&mut self, access_token: String) {
        self.access_token = access_token;
    }

    /// make a get request to the api
    pub fn get(&mut self) -> serde_json::Value {
        // set the oauth
        self.request.set_oauth(self.access_token.clone());

        // add some headers
        self.request.add_header("User-Agent", "Rust Program");
        self.request.add_header("Accept", "application/json");

        self.request.get()
    }
}
