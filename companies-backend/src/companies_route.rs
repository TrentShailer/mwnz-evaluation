use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{api_error::ApiError, fetch_company, try_fetch_company, Company};

/// Transforms a fetch_company::Error into a ApiError
fn fetch_company_to_api_error(error: fetch_company::Error, company_id: i32) -> ApiError {
    match error {
        fetch_company::Error::NotFound(_) => ApiError {
            error: "Company not found".to_string(),
            error_description: format!("No company with id {company_id} exists."),
        },

        fetch_company::Error::StatusCode(code) => {
            log::error!("Failed to fetch company with id {company_id}\n{error}");
            ApiError {
                error: "Failed to fetch company".to_string(),
                error_description: format!(
                    "Unexpected status code was received when fetching company: {code}."
                ),
            }
        }

        fetch_company::Error::Reqwest(_) => {
            log::error!("Failed to fetch company with id {company_id}\n{error}");
            ApiError {
                error: "Failed to fetch company".to_string(),
                error_description: format!(
                    "Encountered an error while fetching company {company_id}\n{error}" // TODO this could be improved
                ),
            }
        }
    }
}

pub async fn get_companies(Path(id): Path<i32>) -> Response {
    let xml = match try_fetch_company(id)
        .await
        .map_err(|e| fetch_company_to_api_error(e, id))
    {
        Ok(xml) => xml,
        Err(e) => return e.into_response(),
    };

    let company = match Company::try_from_xml(xml) {
        Ok(company) => company,
        Err(e) => {
            log::error!("Failed to parse company:\n{e}");

            return ApiError {
                error: "Failed to parse company".to_string(),
                error_description: format!("Could not parse xml for company with id {id}"),
            }
            .into_response();
        }
    };

    (StatusCode::OK, Json(company)).into_response()
}
