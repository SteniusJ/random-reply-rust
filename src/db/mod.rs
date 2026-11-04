use core::fmt;

use reqwest;
use serde_json::Value;

pub struct DbHost<'a> {
    host: &'a str,
    reqwest_client: reqwest::Client,
}

pub enum DbError {
    ReqwestError(reqwest::Error),
    JsonError(serde_json::Error),
    Err(String),
}

impl From<reqwest::Error> for DbError {
    fn from(error: reqwest::Error) -> Self {
        Self::ReqwestError(error)
    }
}

impl From<serde_json::Error> for DbError {
    fn from(error: serde_json::Error) -> Self {
        Self::JsonError(error)
    }
}

impl fmt::Debug for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Db_Error: {:?}", self)
    }
}

impl<'a> DbHost<'a> {
    pub fn new(url: &'a str, password: &'a str) -> Self {
        Self {
            // leaking here is fine since this str lives for the rest of the programs lifetime
            host: format!("{}?password={}", url, password).leak(),
            reqwest_client: reqwest::Client::new(),
        }
    }

    pub async fn get_rand_reply(&self) -> Result<String, DbError> {
        let res = self
            .reqwest_client
            .post(self.host)
            .body("replyMessages[*] | random 1")
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(DbError::Err(format!(
                "DB didn't return OK, code: {}",
                res.status().as_str()
            )));
        }

        let parsed: Value = serde_json::from_str(res.text().await?.as_str())?;
        return Ok(parsed["data"][0]["reply_message"].to_string());
    }

    pub async fn get_rand_reaction(&self) -> Result<String, DbError> {
        let res = self
            .reqwest_client
            .post(self.host)
            .body("reactEmojis[*] | random 1")
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(DbError::Err(format!(
                "DB didn't return OK, code: {}",
                res.status().as_str()
            )));
        }

        let parsed: Value = serde_json::from_str(res.text().await?.as_str())?;
        return Ok(parsed["data"][0]["react_emoji"].to_string());
    }

    pub async fn get_rand_game(&self) -> Result<String, DbError> {
        let res = self
            .reqwest_client
            .post(self.host)
            .body("gameMessages[*] | random 1")
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(DbError::Err(format!(
                "DB didn't return OK, code: {}",
                res.status().as_str()
            )));
        }

        let parsed: Value = serde_json::from_str(res.text().await?.as_str())?;
        return Ok(parsed["data"][0]["react_emoji"].to_string());
    }
}
