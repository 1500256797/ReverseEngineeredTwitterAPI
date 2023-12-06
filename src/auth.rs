use std::{
    fs::File,
    io::{ErrorKind, Read, Write},
    sync::Arc,
};

use super::{ReAPI, BEARER_TOKEN, GUEST_ACTIVE_URL, LOGIN_URL, VERIFY_CREDENTIALS_URL};
use chrono::format;
use reqwest::{
    self,
    cookie::{Cookie, Jar},
    ClientBuilder, Error, Response, Url,
};
use serde::{Deserialize, Serialize};
use serde_json::{self, json};

#[derive(Deserialize)]
pub struct User {
    pub id: i64,
    pub id_str: String,
    pub name: String,
    pub screen_name: String,
}
#[derive(Deserialize)]
pub struct OpenAccount {
    pub user: Option<User>,
    pub next_link: Option<Link>,
    pub attribution_event: Option<String>,
}

#[derive(Deserialize)]
pub struct Subtask {
    pub subtask_id: String,
    pub open_account: Option<OpenAccount>,
}

#[derive(Deserialize)]
pub struct ApiError {
    pub code: i64,
    pub message: String,
}

#[derive(Deserialize)]
pub struct Flow {
    pub errors: Option<Vec<ApiError>>,
    pub flow_token: String,
    pub status: String,
    pub subtasks: Vec<Subtask>,
    pub js_instrumentation: Option<Insrumentation>,
}

#[derive(Deserialize)]
pub struct Insrumentation {
    pub url: String,
    pub timeout_ms: i64,
    pub next_link: Link,
}

#[derive(Deserialize)]
pub struct Link {
    pub link_type: String,
    pub link_id: String,
}

#[derive(Deserialize)]
pub struct GuestToken {
    pub guest_token: String,
}

#[derive(Deserialize)]
pub struct VerifyCredentials {
    pub errors: Option<Vec<ApiError>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CookieData {
    name: String,
    value: String,
}

fn save_cookies_to_file(
    response: &Response,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get the cookies from the response
    let cookies = response.cookies();

    // Serialize the cookies into JSON format
    // Convert cookies into a serializable data structure
    let cookie_data: Vec<CookieData> = cookies
        .map(|cookie| CookieData {
            name: cookie.name().to_string(),
            value: cookie.value().to_string(),
            // Add any other fields you need from the cookie
        })
        .collect();

    // Serialize the cookie data into JSON format
    let serialized_cookies = serde_json::to_string(&cookie_data)?;

    // Open the file for writing
    let mut file = File::create(file_path)?;

    // Write the serialized cookies to the file
    file.write_all(serialized_cookies.as_bytes())?;

    Ok(())
}

impl ReAPI {
    pub fn new() -> ReAPI {
        let client = reqwest::ClientBuilder::new()
            .cookie_store(true)
            .build()
            .unwrap();
        return ReAPI {
            client,
            csrf_token: String::from(""),
            guest_token: String::from(""),
        };
    }

    pub async fn load_from_cookies_data(
        cookies: &Vec<CookieData>,
    ) -> Result<ReAPI, Box<dyn std::error::Error>> {
        // cookie = "foo=bar; Domain=yolo.local";
        let mut csrf_token = "".to_string();
        // Create a new cookie jar and add the cookies to it
        let cookie_jar = reqwest::cookie::Jar::default();
        for cookie in cookies {
            // cookie = "foo=bar; Domain=yolo.local";
            if cookie.name.eq("ct0") {
                csrf_token = cookie.clone().value;
            }
            let cookie_str = format!("{}={}; Domain=twitter.com", cookie.name, cookie.value);
            let url = Url::parse("https://twitter.com")?;
            cookie_jar.add_cookie_str(&cookie_str, &url);
        }

        // Create a reqwest client builder
        let cookie_jar_arc = Arc::new(cookie_jar);
        let client_builder = ClientBuilder::new().cookie_provider(cookie_jar_arc);
        // Build the client
        let client = match client_builder.build() {
            Ok(client) => client,
            Err(err) => {
                eprintln!("Error building client: {}", err);
                return Err(err.into());
            }
        };
        Ok(ReAPI {
            client,
            csrf_token: csrf_token.to_string(),
            guest_token: String::from(""),
        })
    }

    pub fn load_from_cookies_file() -> Result<ReAPI, Box<dyn std::error::Error>> {
        let file_path = "cookies.json";

        // Load the cookies from the file
        // Open the file for reading
        let mut file = File::open(file_path)?;

        // Read the contents of the file into a string
        let mut cookies_json = String::new();
        file.read_to_string(&mut cookies_json)?;

        // Deserialize the cookies from JSON format
        let cookies: Vec<CookieData> = serde_json::from_str(&cookies_json)?;
        let mut csrf_token = "".to_string();
        // Create a new cookie jar and add the cookies to it
        let cookie_jar = reqwest::cookie::Jar::default();
        println!("cookies len: {}", cookies.len());
        for cookie in cookies {
            // cookie = "foo=bar; Domain=yolo.local";
            if cookie.name.eq("ct0") {
                csrf_token = cookie.clone().value;
            }
            let cookie_str = format!("{}={}; Domain=twitter.com", cookie.name, cookie.value);
            let url = Url::parse("https://twitter.com")?;
            cookie_jar.add_cookie_str(&cookie_str, &url);
        }

        // Create a reqwest client builder
        let cookie_jar_arc = Arc::new(cookie_jar);
        let client_builder = ClientBuilder::new().cookie_provider(cookie_jar_arc);
        // Build the client
        let client = match client_builder.build() {
            Ok(client) => client,
            Err(err) => {
                eprintln!("Error building client: {}", err);
                return Err(err.into());
            }
        };

        Ok(ReAPI {
            client,
            csrf_token: csrf_token.to_string(),
            guest_token: String::from(""),
        })
    }

    async fn get_flow(&mut self, body: serde_json::Value) -> Result<Flow, Error> {
        if self.guest_token.is_empty() {
            // generate guest token for login
            self.get_guest_token().await?
        }
        let res = self
            .client
            .post(LOGIN_URL)
            .header("Authorization", format!("Bearer {}", BEARER_TOKEN))
            .header("Content-Type", "application/json")
            .header("User-Agent", "TwitterAndroid/99")
            .header("X-Guest-Token", self.guest_token.replace("\"", ""))
            .header("X-Twitter-Auth-Type", "OAuth2Client")
            .header("X-Twitter-Active-User", "yes")
            .header("X-Twitter-Client-Language", "en")
            .json(&body)
            .send()
            .await?;
        // save cookies to file
        save_cookies_to_file(&res, "cookies.json");

        let cookies = res.cookies();
        for cookie in cookies {
            if cookie.name().eq("ct0") {
                self.csrf_token = cookie.value().to_string()
            }
        }
        let result: Flow = res.json().await?;
        return Ok(result);
    }

    async fn get_flow_token(&mut self, data: serde_json::Value) -> Result<String, String> {
        let res = self.get_flow(data);
        match res.await {
            Ok(info) => {
                if info.subtasks.len() > 0 {
                    let subtask_id = info.subtasks[0].subtask_id.as_str();
                    match subtask_id {
                        "LoginEnterAlternateIdentifierSubtask"
                        | "LoginAcid"
                        | "LoginTwoFactorAuthChallenge"
                        | "DenyLoginSubtask" => {
                            return Err(format!("Auth error: {}", subtask_id));
                        }
                        _ => return Ok(info.flow_token),
                    }
                }
                return Ok(info.flow_token);
            }
            Err(e) => Err(format!("Request error: {}", e.to_string())),
        }
    }

    async fn get_guest_token(&mut self) -> Result<(), Error> {
        let token = format!("Bearer {}", BEARER_TOKEN);
        let res = self
            .client
            .post(GUEST_ACTIVE_URL)
            .header("Authorization", token)
            .send()
            .await;
        match res {
            Ok(r) => {
                let op = r.json::<serde_json::Value>().await?;
                let guest_token = op.get("guest_token");
                if let Some(guest_token) = guest_token {
                    // set guest token for reapi instance
                    self.guest_token = guest_token.to_string();
                    return Ok(());
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
    pub async fn login(
        &mut self,
        user_name: &str,
        password: &str,
        confirmation: &str,
    ) -> Result<String, String> {
        // flow start
        let data = json!(
            {
                "flow_name": "login",
                "input_flow_data": {
                    "flow_context" : {
                        "debug_overrides": {},
                        "start_location": {
                            "location": "splash_screen"
                        }
                    }
                }
            }
        );
        let flow_token = self.get_flow_token(data).await?;

        // flow instrumentation step
        let data = json!(
            {
                "flow_token": flow_token,
                "subtask_inputs" : [{
                    "subtask_id": "LoginJsInstrumentationSubtask",
                    "js_instrumentation":{
                        "response": "{}",
                        "link": "next_link"
                    }
                }],
            }
        );
        let flow_token = self.get_flow_token(data).await?;

        // flow username step
        let data = json!(
            {
                "flow_token": flow_token,
                "subtask_inputs" : [{
                    "subtask_id": "LoginEnterUserIdentifierSSO",
                    "settings_list": {
                        "setting_responses" : [{
                            "key":           "user_identifier",
                            "response_data": {
                                "text_data" :{
                                    "result": user_name
                                }
                            }
                        }],
                        "link": "next_link"
                    }
                }]
            }
        );
        let flow_token = self.get_flow_token(data).await?;

        // flow password step
        let data = json!(
            {
                "flow_token": flow_token,
                "subtask_inputs": [{
                    "subtask_id":     "LoginEnterPassword",
                    "enter_password": {
                        "password": password,
                        "link": "next_link"
                    },
                }]
            }
        );
        let flow_token = self.get_flow_token(data).await?;

        // flow duplication check
        let data = json!(
            {
                "flow_token": flow_token,
                "subtask_inputs": [{
                    "subtask_id":              "AccountDuplicationCheck",
                    "check_logged_in_account": {
                        "link": "AccountDuplicationCheck_false"
                    },
                }]
            }
        );
        let flow_token = self.get_flow_token(data).await;

        match flow_token {
            Err(e) => {
                let mut confirmation_subtask = "";
                for item in vec!["LoginAcid", "LoginTwoFactorAuthChallenge"] {
                    if e.contains(item) {
                        confirmation_subtask = item;
                        break;
                    }
                }
                if !confirmation_subtask.is_empty() {
                    if confirmation.is_empty() {
                        let msg = format!(
                            "confirmation data required for {}",
                            confirmation_subtask.to_owned()
                        );
                        return Err(msg);
                    }
                    let data = json!(
                        {
                            "flow_token": "",
                            "subtask_inputs": {
                                    "subtask_id": confirmation_subtask,
                                    "enter_text": {
                                        "text": confirmation,
                                        "link": "next_link",
                                    },
                            },
                        }
                    );
                    return self.get_flow_token(data).await;
                }
                Ok("".to_owned())
            }
            Ok(_) => return Ok("".to_owned()),
        }
    }

    pub async fn is_logged_in(&mut self) -> bool {
        let req = self
            .client
            .get(VERIFY_CREDENTIALS_URL)
            .header("Authorization", format!("Bearer {}", BEARER_TOKEN))
            .header("X-CSRF-Token", self.csrf_token.to_owned())
            .build()
            .unwrap();
        let res = self.client.execute(req).await.unwrap();
        let cookies = res.cookies();
        for cookie in cookies {
            println!("Name: {}, Value: {}", cookie.name(), cookie.value());
            if cookie.name().eq("ct0") {
                self.csrf_token = cookie.value().to_string()
            }
        }
        let text = res.text().await.unwrap();
        let res: VerifyCredentials = serde_json::from_str(&text).unwrap();
        res.errors.is_none()
    }

    pub async fn get_csrf_token(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let req = self
            .client
            .get(VERIFY_CREDENTIALS_URL)
            .header("Authorization", format!("Bearer {}", BEARER_TOKEN))
            .header("X-CSRF-Token", self.csrf_token.to_owned())
            .build()
            .unwrap();
        let res = self.client.execute(req).await.unwrap();
        let cookies = res.cookies();
        for cookie in cookies {
            println!("Name: {}, Value: {}", cookie.name(), cookie.value());
            // if cookie.name().eq("ct0") {
            //     self.csrf_token = cookie.value().to_string();
            //     return Ok(self.csrf_token.clone());
            // }
        }
        Ok("".to_owned())
    }
}
