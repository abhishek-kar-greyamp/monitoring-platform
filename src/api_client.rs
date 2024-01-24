use std::ops::Sub;

use base64::prelude::*;
use chrono::{Duration, Local};

use crate::model::ContentData;

#[derive(Clone)]
pub struct ApiClient {
    pat: String,
    org_url: String,
    client: reqwest::Client,
}

impl ApiClient {
    pub fn new(pat: String, org_url: String) -> ApiClient {
        return ApiClient {
            pat,
            org_url,
            client: reqwest::Client::new(),
        };
    }
    #[tokio::main]
    pub async fn get_commits(
        self,
        pid: String,
        repo_id: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let date = Local::now();
        let diff = Duration::days(7);

        let end_date = date.sub(diff);

        let end_date_formatted = date.format("%m/%d/%Y %H:%M:%S %p").to_string();
        let start_date_formatted = end_date.format("%m/%d/%Y %H:%M:%S %p").to_string();

        let commit_url = format!("{org_url}/{pid}/_apis/git/repositories/{repo_id}/commits?searchCriteria.fromDate={start_date_formatted}&searchCriteria.toDate={end_date_formatted}&api-version=7.2-preview.2", org_url=self.org_url, pid=pid.as_str(), repo_id=repo_id, start_date_formatted=start_date_formatted, end_date_formatted=end_date_formatted);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Authorization",
            format!(
                "Basic {}",
                BASE64_STANDARD.encode(format!("{}:{}", "", self.pat))
            )
            .parse()?,
        );
        let request = self
            .client
            .request(reqwest::Method::GET, commit_url)
            .headers(headers);
        let response = request.send().await?;

        let body = response.json::<serde_json::Value>().await?;

        Ok(body)
    }

    #[tokio::main]
    pub async fn write_wiki(
        self,
        wiki_data: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let dt = Local::now().to_rfc3339();
        let wiki_page_url = format!(
            "{}/{}/_apis/wiki/wikis/{}/pages?path={}&api-version=7.2-preview.1",
            self.org_url, "TestBank", "TestBank.wiki", dt
        );
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Authorization",
            format!(
                "Basic {}",
                BASE64_STANDARD.encode(format!("{}:{}", "", self.pat))
            )
            .parse()?,
        );
        headers.insert("Content-Type", "application/json".parse()?);
        let data = ContentData::new(wiki_data);
        let display_data = data.dispaly();
        let request = self
            .client
            .request(reqwest::Method::PUT, wiki_page_url)
            .body(display_data)
            .headers(headers);

        let response = request.send().await?;

        let status = response.status();

        let is_client_error = status.is_client_error();

        if is_client_error {
            return Err(Box::from("Wiki not found"));
        }

        let status = response.json::<serde_json::Value>().await?;
        Ok(status)
    }

    #[tokio::main]
    pub async fn get_projects(self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let projects_url = format!("{}/_apis/projects?api-version=7.2-preview.4", self.org_url);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Authorization",
            format!(
                "Basic {}",
                BASE64_STANDARD.encode(format!("{}:{}", "", self.pat))
            )
            .parse()?,
        );
        let request = self
            .client
            .request(reqwest::Method::GET, projects_url)
            .headers(headers);

        let response = request.send().await?;
        let body = response.json::<serde_json::Value>().await?;

        Ok(body)
    }
    #[tokio::main]
    pub async fn get_repositories(
        self,
        pid: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let repositories_url = format!(
            "{}/{}/_apis/git/repositories?api-version=7.2-preview.1",
            self.org_url, pid
        );

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Authorization",
            format!(
                "Basic {}",
                BASE64_STANDARD.encode(format!("{}:{}", "", self.pat))
            )
            .parse()?,
        );
        let request = self
            .client
            .request(reqwest::Method::GET, repositories_url)
            .headers(headers);

        let response = request.send().await?;
        let body = response.json::<serde_json::Value>().await?;

        Ok(body)
    }
}
