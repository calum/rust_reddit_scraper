use requester::Requester;

pub struct Token {
    url: String,
    access_token: String
}

impl Token {
    /// Generate a new access token for the reddit api
    pub fn new(url: String, app_username: String, app_password: String, username: String, userpassword: String) -> Result<Token, String> {
        // create the Post request
        let mut http_post_request = Requester::new(url.clone());

        // set the body
        let body = format!("grant_type=password&username={}&password={}", username, userpassword);
        http_post_request.set_data(body.clone());

        // add some headers
        http_post_request.add_header("Content-Type", "application/x-www-form-urlencoded");
        http_post_request.add_header("User-Agent", "Rust Program");
        http_post_request.add_header("Accept", "application/json");
        http_post_request.add_header("Content-Length", &format!("{}",body.len()));

        // add the basic authentication
        http_post_request.set_authorization(app_username, app_password);

        // send the request
        let response = http_post_request.post();

        // return the access token
        if let Some(access_token) = response["access_token"].as_str() {
            return Ok(Token {
                url: url,
                access_token: String::from(access_token)
            });
        }

        // otherwise an error is returned
        Err(format!("Failed to get an access token from the response: {:?}", response))
    }

    // return a clone of the access token
    pub fn get_access_token(&self) -> String {
        self.access_token.clone()
    }
}
