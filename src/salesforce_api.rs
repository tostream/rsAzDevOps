
use reqwest::header::HeaderMap;
use reqwest::blocking::{Client,Response};
use serde::{Serialize, Deserialize};
use std::thread;
use std::time::Duration;
use std::error::Error;
pub mod base_api;

// data type //

#[derive(Debug, Serialize, Deserialize)]
pub struct SfQuery {
    pub operation:String,
    pub query: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum QueryState {
    UploadComplete,
    InProgress,
    Aborted,
    JobComplete,
    Failed
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryJob {
    pub id: String,
    #[serde(rename = "state")]
    pub state: QueryState
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub signature: String,
    pub instance_url: String,
    pub id: String,
    pub token_type: String,
    pub issued_at: String
}

// helper function //

fn parse_and_convert(input: &str,pattern:String) -> String {
    input.replace(pattern.as_str(), " ")
}

fn split_string(input: &str) -> String{
    let mut parts: Vec<&str> = input.split('\n').collect();
    let mut res:Vec<String>= Vec::new();
    parts.remove(0);
    for part in parts {
        match part.chars().next() {
            Some(first_char) if first_char == '"' => {
                    res.push(parse_and_convert(parse_and_convert(part,'\"'.to_string()).as_str(),'"'.to_string()))
            },
            _ => println!("not this one"),
        }
    }
    res.join(",")
}

fn get_sf_header() -> HeaderMap{
    let mut http_header = HeaderMap::new();
    http_header.insert(reqwest::header::CONTENT_TYPE,"application/json".parse().unwrap());
    http_header.insert(reqwest::header::ACCEPT,"application/json".parse().unwrap());
    http_header
}

fn get_sf_query(query_str: String) -> SfQuery{
    SfQuery{
        operation :"query".to_string(),
        query:query_str}
}

pub fn get_query_id(response: Response) -> QueryJob{
    let quert_return: QueryJob = response.json().unwrap();
    quert_return
}

pub fn get_token(response: Response) -> String{
    let token: AccessToken = response.json().unwrap();
    token.access_token
}

// Main function //

pub fn request_access_token(url: &str, username: String, password: String) -> Result<Response, reqwest::Error>{
    let mut http_header = HeaderMap::new();
    // http_header.
    http_header.insert(reqwest::header::CONTENT_TYPE,"application/x-www-form-urlencoded".parse().unwrap());
    let _url = format!("{}{}" ,url ,"/oauth2/token?grant_type=client_credentials");
    let http_client = Client::new();
    http_client
    .post(_url)
    .headers(http_header)
    .basic_auth(username, Some(password))
    .send()
}

pub fn request_query_start(
    base_url: base_api::BearerApi, 
    query_str: String
) -> Result<Response, reqwest::Error>{
    let http_client = Client::new();
    let _url = format!("{}/data/v55.0/jobs/query/",base_url.url.clone());

    // http_header.
    let http_header = get_sf_header();
    
    // salesforce query]
    let t_sf_q = get_sf_query(query_str);
    // println!("{:?}",&t_sf_q);

    http_client
    .post(_url)
    .headers(http_header)
    .bearer_auth(&base_url.token)
    .json(&t_sf_q)
    .send()
}


pub fn request_query_state(
    base_url: BearerApi, 
    query_job_id: &str
) -> Result<Response, reqwest::Error>{
    let http_client = Client::new();

    // http_header.
    let http_header = get_sf_header();
    
    // salesforce query
    // let t_sf_q = get_sf_query(query_str);
    let _url = format!("{}/data/v55.0/jobs/query/{}",base_url.url.clone() , query_job_id);

    http_client
    .get(_url)
    .headers(http_header)
    .bearer_auth(&base_url.token)
    // .json(&t_sf_q)
    .send()
}

pub fn request_query_result(
    base_url: base_api::BearerApi, 
    query_job_id: &str,
    locator: Option<String>
)  -> Result<Response, reqwest::Error>{
    let http_client = Client::new();
    let http_header = get_sf_header();
    let _url = format!("{}/data/v55.0/jobs/query/{}",base_url.url.clone() , query_job_id);
    let full_url = match locator {
        Some(locator) => _url + "/results?params=" + locator.as_str(),
        None => _url + "/results",
    };
    http_client
    .get(full_url)
    .headers(http_header)
    .bearer_auth(&base_url.token)
    .send()
}

pub fn salesforce_data_ready(
    state_url: BearerApi, job: QueryJob) -> Option<QueryJob>{

    let query_id_result = match job.state {
                
        QueryState::Aborted | QueryState::JobComplete |
        QueryState::UploadComplete | QueryState::Failed => Ok(job.id),
        _ => Err(()),
    };

    match query_id_result {
        Ok(query_id) => {
            println!("{:#?}", &query_id);
            // let res_contentmatch = 
            loop {
                // let check_state_obj = salesforce_api::request_query_state(state_url.clone(), &query_id);
                match request_query_state(state_url.clone(), &query_id) {
                    Ok(response) => {
                        let state_result = get_query_id(response);
                        match state_result.state {
                            QueryState::Aborted |
                            QueryState::JobComplete |
                            QueryState::Failed => {
                                println!("Query completed with state: {:?}", state_result.state);
                                break Some(QueryJob{id:query_id,state:state_result.state})
                                // break;  // Exit the loop if the query is finished
                            },
                            _ => thread::sleep(Duration::from_secs(5)),  // Otherwise, sleep and repeat
                        }
                    },
                    Err(e) => {
                        println!("Failed to check query state. 1 {:?}" , e);
                        break None
                        // break;  // Exit on failure to get the state
                    },
                }
            }
        },
        Err(_) => {
            println!("Failed to check query state. 2 ");
            None
        },
    }
    
}

pub fn get_full_sf_content(base_url: BearerApi, query_job_id: &str) -> Result<String, Box<dyn Error>> {
    let mut locator: Option<String> = None;
    let mut full_content = String::new();

    loop {
        let response = match request_query_result(base_url.clone(), query_job_id, locator.clone()) {
            Ok(response) => response,
            Err(e) => break println!("Somethings wrong: {} ", e),
        };
        
        // Extract headers before consuming response
        let next_locator = response.headers().get("sforce-locator").map(|v| v.to_str().unwrap_or("").to_string());
        let content_type = response.headers().get("content-type").map(|v| v.to_str().unwrap_or("").to_string());

        // Consume response to get the text
        // let text = response.text()?;
        let content_text = match response.text() {
            Ok(content_text) => content_text,
            Err(e) => break println!("Somethings wrong: {} ", e),
        };
        // println!("locator check text : {:#?}", &content_text);
        match content_type {
            Some(content_type) => {
                if content_type == "text/csv" {
                    full_content.push_str(&content_text);
                }
            },
            None => break,
        }
        
        if let Some(locator_str) = next_locator {
            if !locator_str.is_empty() {
                locator = Some(locator_str);
            } else {
                break;
            }
        } else {
            break;
        }
    }

    Ok(full_content)
}

///data/v55.0/jobs/query/
pub fn get_sf_table_content(state_url:BearerApi,table_name: &str, filter_query:String ) -> Option<String>{

    let query_res: Result<reqwest::blocking::Response, reqwest::Error>  = request_query_start(
        state_url.clone(),
        format!("SELECT QualifiedApiName FROM EntityParticle WHERE EntityDefinition.QualifiedApiName='{}' AND IsCompound=false",table_name)
    );
    if query_res.is_ok(){

        let result_obj = get_query_id(query_res.ok().unwrap());
        let res_content = salesforce_data_ready(state_url.clone(),result_obj);
        
        let query = match res_content{
            Some(res_id) => {
                let result_content = get_full_sf_content(state_url.clone(),  &res_id.id);
                match result_content {
                    Ok(res_content) => res_content,
                    Err(_) => panic!("Failed to check query state. 1 "),
                }
            },
            None => panic!("Failed to check query state. 1 "),
        };
        // let quert_str = parse_and_convert(&query);
        let quert_str = split_string(&query);
        let table_res: Result<reqwest::blocking::Response, reqwest::Error>  = request_query_start(
            state_url.clone(),
            format!("SELECT {} FROM {} {} ",quert_str,table_name,filter_query)
        ); 
        if table_res.is_ok(){
            let table_obj = get_query_id(table_res.ok().unwrap());
            let table_content = salesforce_data_ready(state_url.clone(),table_obj);
            
            let table_result = match table_content{
                Some(res_id) => {
                    let result_content = get_full_sf_content(state_url.clone(),  &res_id.id);
                    match result_content {
                        Ok(res_content) => res_content,
                        Err(_) => panic!("Failed to check query state. 1 "),
                    }
                },
                None => panic!("Failed to check query state. 1 "),
            };
            return Some(table_result);
        }
    }
    None
}


pub fn get_sf_table_main(
    url :String,
    username :String,
    password :String,
    table_name :String,
    filter_query :Option< String>) -> Option<String>{
    let login_res = request_access_token(&url.as_str(),username,password);
    let _filter_query = match filter_query {
        Some(query) => query,
        None => "".to_string(),
    };
    if login_res.is_ok(){
        let access_token = get_token(login_res.ok().unwrap());
        let _state_url = BearerApi{
            url :url,
            token : access_token
        };
        return get_sf_table_content(_state_url,table_name.as_str(),_filter_query)
    }
    None
}
