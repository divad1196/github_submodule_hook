use anyhow::{anyhow, Result};
use rocket::{
    data::{FromData, Outcome, ToByteUnit},
    request::Request,
    serde::DeserializeOwned,
    Data,
};
use serde_json;


// use core::{future::Future, pin::Pin};

pub async fn data_to_json<'a>(data: Data<'a>) -> Result<serde_json::Value> {
    let json_str = match data.open(128.kilobytes()).into_string().await {
        Ok(d) => d.value.clone(),
        Err(_) => {
            return Err(anyhow!("Error"));
        }
    };
    match serde_json::from_str(&json_str) {
        Ok(value) => Ok(value),
        Err(e) => Err(anyhow!(e)),
    }
}

pub struct Json<T> {
    pub data: T,
}

#[rocket::async_trait]
impl<'r, T: DeserializeOwned> FromData<'r> for Json<T> {
    type Error = anyhow::Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        println!("{:?}", req.headers());
        match data_to_json(data).await {
            Ok(value) => match serde_json::from_value::<T>(value) {
                Ok(data) => Outcome::Success(Json::<T> { data }),
                Err(e) => Outcome::Failure((rocket::http::Status::ExpectationFailed, anyhow!(e)))
            },
            Err(e) => Outcome::Failure((rocket::http::Status::ExpectationFailed, e))
        }
    }
}

pub struct OptionalJson<T> {
    pub data: Option<T>,
}

#[rocket::async_trait]
impl<'r, T: DeserializeOwned> FromData<'r> for OptionalJson<T> {
    type Error = anyhow::Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        println!("{:?}", req.headers());
        match data_to_json(data).await {
            Ok(value) => match serde_json::from_value::<T>(value) {
                Ok(data) => Outcome::Success(OptionalJson::<T> { data: Some(data) }),
                Err(e) => {
                    eprintln!("Deserialization error: {:?}", e);
                    Outcome::Success(OptionalJson::<T> { data: None })
                }
            },
            Err(e) => Outcome::Failure((rocket::http::Status::ExpectationFailed, e))
        }
    }
}


#[derive(Debug)]
pub struct JsonBody(pub serde_json::Value);

impl JsonBody {
    pub fn print(&self) {
        println!("{}", serde_json::to_string_pretty(&self.0).unwrap());
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for JsonBody {
    type Error = anyhow::Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        println!("{:?}", req.headers());
        match data_to_json(data).await {
            Ok(value) => Outcome::Success(JsonBody(value)),
            Err(e) => {
                return Outcome::Failure((rocket::http::Status::ExpectationFailed, e));
            }
        }
    }
}
