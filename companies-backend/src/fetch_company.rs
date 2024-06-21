use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("No company with the ID exists {0}")]
    NotFound(i32),

    #[error("Recieved unexpected status code {0}")]
    StatusCode(StatusCode),

    #[error("Failed to make Reqwest request\n{0}")]
    Reqwest(#[from] reqwest::Error),
}

/// Performs a GET request to
/// https://raw.githubusercontent.com/MiddlewareNewZealand/evaluation-instructions/main/xml-api/{id}.xml
///
/// Interprets body as text and returns body or an error
pub async fn try_fetch_company(id: i32) -> Result<String, Error> {
    let url = format!("https://raw.githubusercontent.com/MiddlewareNewZealand/evaluation-instructions/main/xml-api/{id}.xml");

    let response = reqwest::get(url).await?;
    match response.status() {
        StatusCode::NOT_FOUND => Err(Error::NotFound(id)),
        StatusCode::OK => Ok(response.text().await?), // Get body on OK, propegate reqwest errors up
        other_code => Err(Error::StatusCode(other_code)), // Other statuses are unexpected, we should error, this could be handled much better
    }
}
