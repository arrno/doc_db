use reqwest::{
    header::{HeaderMap, HeaderValue},
    Response,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::Error;

pub async fn do_api() {
    let client_info = APIArgs {
        headers: vec![],
        base_url: String::from("localhost"),
        port: 8080,
        key: None,
    };
    let client = APIMGR::new(client_info).await.unwrap();
    let mut target = client.base_target();
    // target.push_str("/cloud-rule");
    target.push_str("/document");
    println!("{}", &target);
    let payload = json!({
            "path": "home.notes.rust",
            "value": "Hello, World!",
        }
    );
    let r = client.post(&target, &payload).await.unwrap();
    client.display_response(r).await.unwrap();
    //
    let mut target = client.base_target();
    target.push_str("/document?path=home.notes.rust");
    let r = client.get_resp(&target).await.unwrap();
    client.display_response(r).await.unwrap();
}

// -------------------------------------------------------------------------------

struct Headers(&'static str, &'static str);

struct APIArgs {
    headers: Vec<Headers>,
    base_url: String,
    port: u32,
    key: Option<String>,
}

struct APIMGR {
    base_url: String,
    headers: HeaderMap,
    client: reqwest::Client,
    port: u32,
}

impl APIMGR {
    async fn new(args: APIArgs) -> Result<Self, Box<dyn Error>> {
        let APIArgs {
            headers,
            base_url,
            port,
            key,
        } = args;
        let mut hmap = HeaderMap::new();
        headers.iter().for_each(|header| {
            hmap.insert(header.0, HeaderValue::from_str(header.1).unwrap());
        });
        if let Some(token) = key {
            hmap.insert("Authorization", HeaderValue::from_str(&token).unwrap());
        }
        Ok(APIMGR {
            headers: hmap,
            client: reqwest::Client::new(),
            port: port,
            base_url: base_url,
        })
    }

    fn base_target(&self) -> String {
        format!("http://{}:{}", &self.base_url, &self.port,)
    }

    async fn display_response(&self, r: Response) -> Result<(), Box<dyn Error>> {
        let status = r.status();
        println!("{:?}", &status);
        let text = r.text().await?;
        if vec!["400", "404", "403", "500"].contains(&status.as_str()) {
            println!("{text}");
        }
        let resp: Value = serde_json::from_str(&text).unwrap_or("{}".into());
        let pretty_resp = serde_json::to_string_pretty(&resp)?;
        println!("{}", pretty_resp);
        Ok(())
    }

    async fn get<T>(&self, uri: &str) -> Result<T, Box<dyn Error>>
    where
        T: for<'a> Deserialize<'a>,
    {
        let data = self
            .client
            .get(uri)
            .headers(self.headers.clone())
            .send()
            .await?
            .json::<T>()
            .await?;
        Ok(data)
    }

    async fn get_resp(&self, uri: &str) -> Result<Response, Box<dyn Error>> {
        let resp = self
            .client
            .get(uri)
            .headers(self.headers.clone())
            .send()
            .await?;
        Ok(resp)
    }

    async fn post<T>(&self, uri: &str, payload: &T) -> Result<Response, Box<dyn Error>>
    where
        T: Serialize,
    {
        let json = serde_json::to_string(payload)?;
        let resp = self
            .client
            .post(uri)
            .headers(self.headers.clone())
            .header("Content-Type", "application/json")
            .body(json)
            .send()
            .await?;
        Ok(resp)
    }

    async fn patch<T>(&self, uri: &str, payload: &T) -> Result<Response, Box<dyn Error>>
    where
        T: Serialize,
    {
        let json = serde_json::to_string(payload)?;
        let resp = self
            .client
            .patch(uri)
            .headers(self.headers.clone())
            .header("Content-Type", "application/json")
            .body(json)
            .send()
            .await?;
        Ok(resp)
    }

    async fn delete(&self, uri: &str) -> Result<Response, Box<dyn Error>> {
        let resp = self
            .client
            .delete(uri)
            .headers(self.headers.clone())
            .send()
            .await?;
        Ok(resp)
    }
    // TODO
    // fn put() {}
}
