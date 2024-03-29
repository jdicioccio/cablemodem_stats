use crate::fetcher::Fetcher;
use isahc::prelude::*;
use super::make_request_builder;

pub struct CGM4331COM {
    username: String,
    password: String,
}

impl CGM4331COM {
    pub fn new(username: &String, password: &String) -> CGM4331COM {
        CGM4331COM {
            username: String::from(username),
            password: String::from(password),
        }
    }
}

impl Fetcher for CGM4331COM {
    fn fetch(&self, use_ssl: bool) -> Result<String, isahc::Error> {
        let req_builder = make_request_builder("10.0.0.1/check.jst", use_ssl);

        let req = req_builder
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(format!(
                "username={}&password={}",
                self.username, self.password
            ))?;

        let resp = req.send()?;

        assert!(resp.status().is_redirection());

        let mut sess_cookie: Option<&str> = None;

        for (k, v) in resp.headers() {
            if k.as_str() == "set-cookie" {
                if let Ok(cookie) = v.to_str() {
                    if cookie.starts_with("DUKSID=") {
                        sess_cookie = Some(cookie.split_once(';').unwrap().0);
                    }
                }
            }
        }

        let cookie = sess_cookie.unwrap();

        let req2_builder = make_request_builder("10.0.0.1/network_setup.jst", use_ssl);
        let req2 = req2_builder.method("GET").header("Cookie", cookie).body(())?;
        let mut resp2 = req2.send()?;

        assert!(resp2.status().is_success());

        let body2 = resp2.text()?;

        Ok(body2)
    }
}
