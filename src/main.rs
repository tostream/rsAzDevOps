
pub mod base_api;
pub mod salesforce_api;

fn main() {

    /*
        url :string
        username :string
        password :string
        table_name :string
        filter_query :option string
     */
    let login_res = salesforce_api::request_access_token(
        "",
        ", 
        ";


    if login_res.is_ok(){
        let access_token = salesforce_api::get_token(login_res.ok().unwrap());
        // let result_obj = salesforce_api::get_query_id(res.ok().unwrap());
        println!("{:#?}", &access_token);
        let _state_url = salesforce_api::BearerApi{
            url :"".to_string(),
            token : access_token.clone()
        };
        let result = salesforce_api::get_sf_table_content(_state_url,"Account_Profile__c","".to_string());
        // println!("{:?}",result);
    }
}

// fn get_sf_table_content(access_token:String,table_name: &str, filter_query:String ){
//     let state_url = salesforce_api::BearerApi{
//         url :"".to_string(),
//         token : access_token
//     };
//     let query_res: Result<reqwest::blocking::Response, reqwest::Error>  = salesforce_api::request_query_start(
//         state_url.clone(),
//         format!("SELECT QualifiedApiName FROM EntityParticle WHERE EntityDefinition.QualifiedApiName='{}' AND IsCompound=false",table_name)
//     );
//     if query_res.is_ok(){

//         let result_obj = salesforce_api::get_query_id(query_res.ok().unwrap());
//         let res_content = salesforce_api::salesforce_data_ready(state_url.clone(),result_obj);
        
//         let query = match res_content{
//             Some(res_id) => {
//                 let result_content = salesforce_api::get_full_sf_content(state_url.clone(),  &res_id.id);
//                 match result_content {
//                     Ok(res_content) => res_content,
//                     Err(_) => panic!("Failed to check query state. 1 "),
//                 }
//             },
//             None => panic!("Failed to check query state. 1 "),
//         };
//         // let quert_str = parse_and_convert(&query);
//         let quert_str = split_string(&query);
//         let table_res: Result<reqwest::blocking::Response, reqwest::Error>  = salesforce_api::request_query_start(
//             state_url.clone(),
//             format!("SELECT {} FROM {} {} ",quert_str,table_name,filter_query)
//         ); 
//         if table_res.is_ok(){
//             let table_obj = salesforce_api::get_query_id(table_res.ok().unwrap());
//             let table_content = salesforce_api::salesforce_data_ready(state_url.clone(),table_obj);
            
//             let table_result = match table_content{
//                 Some(res_id) => {
                    
//                     let result_content = salesforce_api::get_full_sf_content(state_url.clone(),  &res_id.id);
//                     match result_content {
//                         Ok(res_content) => res_content,
//                         Err(_) => panic!("Failed to check query state. 1 "),
//                     }
//                 },
//                 None => panic!("Failed to check query state. 1 "),
//             };
//             println!("{:?}" ,table_result);
//         }
//     }
// }
// fn get_full_sf_content(base_url: salesforce_api::BearerApi, 
//     query_job_id: &str,
//     locator: Option<String>
//     // mut result_vec: Vec<String>
// ) -> Vec<String>

// pub trait BaseAPI {
//     fn new(base_url: &str, headers: Option<HeaderMap>,body: Option<String>) -> Self;
//     fn base_url(&self) -> &String;
//     fn headers(&self) -> &Option<HeaderMap>;
//     fn body(&self) -> &Option<String>;
// }

// pub fn get_request<T: BaseAPI>(query: T) -> Result<Response,Error>{
//     let http_client = Client::new();
//     let unwrapped_header = query.headers().clone();
//     let base_url = query.base_url();
//     match unwrapped_header {
//         Some(header) => http_client
//                     .get(base_url)
//                     .headers(header).send() ,
//         None => http_client
//         .get(base_url).send(),
//     }
    
// }

// pub struct TestAPi {
//     base_url: String,
//     headers: Option<HeaderMap>,
//     body: Option<String>,
// }
// impl BaseAPI for TestAPi {
//     fn new(base_url: &str, headers: Option<HeaderMap>,body: Option<String>) -> TestAPi {
//         TestAPi {
//             base_url: base_url.to_string(),
//             headers: Some(headers.unwrap_or_default()),
//             body: Some(body.unwrap_or_default()),
//         }
//     }
//     fn base_url(&self) -> &String {
//         &self.base_url
//     }
//     fn headers(&self) -> &Option<HeaderMap> {
//         &self.headers
//     }
//     fn body(&self) -> &Option<String> {
//         &self.body
//     }
// }

