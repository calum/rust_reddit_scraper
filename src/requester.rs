use std::{str, io};
use futures::{Stream, Future};
use hyper::Client;
use hyper::Chunk;
use hyper::Uri;
use hyper::{Method, Request};
use hyper::header::{ContentType, Authorization, Basic, Bearer};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use serde_json::Value;
use serde_json;

#[derive(Debug)]
struct ReqHeader {
    name: String,
    value: String
}

impl ReqHeader {
    pub fn new(name: String, value: String) -> ReqHeader {
        ReqHeader {
            name,
            value
        }
    }
}

#[derive(Debug)]
pub struct Requester {
    url: String,
    data: Option<String>,
    headers: Option<Vec<ReqHeader>>,
    authentication: Option<ReqHeader>,
    oauth: Option<String>
}

impl Requester {
    /// Create a new HTTP Requester
    pub fn new(url: String) -> Requester {
        Requester {
            url: url,
            data: None,
            headers: None,
            authentication: None,
            oauth: None
        }
    }

    /// Set the headers of the HTTP request
    pub fn add_header(&mut self, name: &str, value: &str) {
        match self.headers {
            Some(ref mut values) => values.push(ReqHeader::new(String::from(name), String::from(value))),
            None => {
                self.headers = Some(Vec::new());
                self.add_header(name, value);
            }
        }
    }

    /// set the url
    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    /// Set the authorization header
    pub fn set_authorization(&mut self, username: String, password: String) {
        self.authentication = Some(ReqHeader::new(username, password));
    }

    /// set the bearer token for oauth2
    pub fn set_oauth(&mut self, token: String) {
        self.oauth = Some(token);
    }

    /// Set the body of the HTTP request
    pub fn set_data(&mut self, data: String) {
        self.data = Some(data);
    }

    /// Make a HTTP Get request to the
    /// url set in the constructor.
    pub fn get(&self) -> Value {
        let uri: Uri = self.url.parse().unwrap();

        // create the http client
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle).unwrap())
            .build(&handle);

        // create the GET request
        let mut req = Request::new(Method::Get, uri.clone());

        // set the headers
        if let Some(ref headers) = self.headers {
            for header in headers {
                req.headers_mut().set_raw(header.name.clone(), header.value.clone());
            }
        } else {
            req.headers_mut().set(ContentType::json());
        }

        // set the authorization header
        if let Some(ref authentication) = self.authentication {
            req.headers_mut().set(
                Authorization(
                    Basic {
                        username: authentication.name.clone(),
                        password: Some(authentication.value.clone())
                    }
                )
            );
        }

        // set the oauth header
        if let Some(ref token) = self.oauth {
            req.headers_mut().set(
                Authorization(
                    Bearer {
                        token: token.clone()
                    }
                )
            );
        }

        // prepare the Get request
        let get = client.request(req).and_then(|res| {
            println!("Response status: {}", res.status());

            res.body().concat2().and_then(move |body: Chunk| {
                let v: Value = serde_json::from_slice(&body).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        e
                    )
                }).expect("Failed to parse json");
                Ok(v)
            })
        });

        // send request
        let got = core.run(get).expect("Failed to make get request");

        got
    }

    /// Make a HTTP Post request to the
    /// url set in the constructor using
    /// the string set in the set_data function.
    pub fn post(&self) -> Value {
        let uri: Uri = self.url.parse().unwrap();

        if let Some(ref body) = self.data {
            // create the http client
            let mut core = Core::new().unwrap();
            let handle = core.handle();
            let client = Client::configure()
                .connector(HttpsConnector::new(4, &handle).unwrap())
                .build(&handle);

            // create the POST request
            let mut req = Request::new(Method::Post, uri.clone());

            // set the headers
            if let Some(ref headers) = self.headers {
                for header in headers {
                    req.headers_mut().set_raw(header.name.clone(), header.value.clone());
                }
            } else {
                req.headers_mut().set(ContentType::json());
            }

            // set the authorization header
            if let Some(ref authentication) = self.authentication {
                req.headers_mut().set(
                    Authorization(
                        Basic {
                            username: authentication.name.clone(),
                            password: Some(authentication.value.clone())
                        }
                    )
                );
            }

            // set the oauth header
            if let Some(ref token) = self.oauth {
                req.headers_mut().set(
                    Authorization(
                        Bearer {
                            token: token.clone()
                        }
                    )
                );
            }

            req.set_body(body.clone());

            // prepare the POST request
            let post = client.request(req).and_then(|res| {
                println!("Response status: {}", res.status());

                res.body().concat2().and_then(move |body: Chunk| {
                    let v: Value = serde_json::from_slice(&body).map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::Other,
                            e
                        )
                    }).expect("Failed to parse json");
                    Ok(v)
                })
            });

            // send request
            let posted = core.run(post).expect("Failed to make POST request");

            posted
        } else {
            json!({})
        }
    }
}
