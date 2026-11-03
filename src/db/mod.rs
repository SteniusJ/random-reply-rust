use core::fmt;

use reqwest;
use serde_json::Value;

pub struct Db_Host<'a> {
    host: &'a str,
    reqwest_client: reqwest::Client,
}

pub enum Db_Error {
    ReqwestError(reqwest::Error),
    JsonError(serde_json::Error),
    Err(String),
}

impl From<reqwest::Error> for Db_Error {
    fn from(error: reqwest::Error) -> Self {
        Self::ReqwestError(error)
    }
}

impl From<serde_json::Error> for Db_Error {
    fn from(error: serde_json::Error) -> Self {
        Self::JsonError(error)
    }
}

impl fmt::Debug for Db_Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Db_Error: {:?}", self)
    }
}

impl<'a> Db_Host<'a> {
    pub fn new(url: &'a str, password: &'a str) -> Self {
        Db_Host {
            // leaking here is fine since this str lives for the rest of the programs lifetime
            host: format!("{}?password={}", url, password).leak(),
            reqwest_client: reqwest::Client::new(),
        }
    }

    pub async fn get_rand_reply(&self) -> Result<String, Db_Error> {
        let res = self
            .reqwest_client
            .post(self.host)
            .body("replyMessages[*] | random 1")
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(Db_Error::Err(format!(
                "DB didn't return OK, code: {}",
                res.status().as_str()
            )));
        }

        let parsed: Value = serde_json::from_str(res.text().await?.as_str())?;
        return Ok(parsed["data"][0]["reply_message"].to_string());
    }

    pub async fn get_rand_reaction(&self) -> Result<String, Db_Error> {
        let res = self
            .reqwest_client
            .post(self.host)
            .body("reactEmojis[*] | random 1")
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(Db_Error::Err(format!(
                "DB didn't return OK, code: {}",
                res.status().as_str()
            )));
        }

        let parsed: Value = serde_json::from_str(res.text().await?.as_str())?;
        return Ok(parsed["data"][0]["react_emoji"].to_string());
    }

    pub async fn get_rand_game(&self) -> Result<String, Db_Error> {
        let res = self
            .reqwest_client
            .post(self.host)
            .body("gameMessages[*] | random 1")
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(Db_Error::Err(format!(
                "DB didn't return OK, code: {}",
                res.status().as_str()
            )));
        }

        let parsed: Value = serde_json::from_str(res.text().await?.as_str())?;
        return Ok(parsed["data"][0]["react_emoji"].to_string());
    }
}
