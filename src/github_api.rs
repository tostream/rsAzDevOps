pub mod base_api;

use reqwest::blocking::{Client,Response};

fn get_gh_header() -> HeaderMap{
    let mut http_header = HeaderMap::new();
    http_header.insert(reqwest::header::CONTENT_TYPE,"application/json".parse().unwrap());
    http_header.insert(reqwest::header::ACCEPT,"application/json".parse().unwrap());
    http_header
}
pub struct GithubEnterprise{
    pub url: String,
    pub token: String,
    pub organiztion: String,
    pub repo: String
}
 
pub fn upload_artifact(conn: GithubEnterprise, release_id:String ,file_path:String ,data_file:String )  -> Result<Response, reqwest::Error>{
    let http_header = get_gh_header();
    let http_client = Client::new();
    let _url = format!("{}/releases/{}/assets?name={}",conn.url.clone(),release_id,file_path);
    http_client
    .post(_url)
    .headers(http_header)
    .bearer_auth(&conn.token)
    .data(data_file)
    .send()
}  

pub fn get_release_id(conn: GithubEnterprise, tag:String ) -> Result<Response, reqwest::Error>{
    let http_header = get_gh_header();
    let http_client = Client::new();
    let _url = format!("{}/releases/tags/{}",conn.url.clone(),tag);
    http_client
    .get(_url)
    .headers(http_header)
    .bearer_auth(&conn.token)
    .send()
}

pub fn get_asset_list(conn: GithubEnterprise, release_id:String ) -> Result<Response, reqwest::Error> {
    let http_header = get_gh_header();
    let http_client = Client::new();
    let _url = format!("{}/releases/{}/assets/",conn.url.clone(),release_id);
    http_client
    .get(_url)
    .headers(http_header)
    .bearer_auth(&conn.token)
    .send()

}

pub fn get_asset(conn: GithubEnterprise, asset_id:String )  -> Result<Response, reqwest::Error>{
    let http_header = get_gh_header();
    let http_client = Client::new();
    let _url = format!("{}/releases/assets/{}/",conn.url.clone(),asset_id);
    http_client
    .get(_url)
    .headers(http_header)
    .bearer_auth(&conn.token)
    .send()
}

// class Github(BaseApi):
//     def __init__(self, base_url: str, user: str | None = None, password: str | None = None, auth: str | None = 'basic', **kwargs: any):
//         super().__init__(base_url, user, password, auth, **kwargs)
//         self.base_url = self.base_url + f"repos/{self.USER_OR_ORG}/{self.REPO}/"
//         self.headers['Content-Type'] = "application/octet-stream"
//         self.headers['application'] = "application/vnd.github+json"
    
//     def upload_artifact(self,RELEASE_ID,FILE_PATH,data_file) -> Optional[Response]:
//         param= f"/releases/{RELEASE_ID}/assets?name={FILE_PATH}"
//         return self.post(param,self.headers,data=data_file)
    
//     def get_release_id(self,tag:str) -> Optional[Response]:
//         param= f"/releases/tags/{tag}"
//         return self.get(param,self.headers)

//     def get_asset_list(self,release_id:str) -> Optional[Response]:
//         param = f"/releases/{release_id}/assets"
//         return self.get(param,self.headers)
    
//     def get_asset(self,asset_id:str) -> Optional[Response]:
//         param = f"/releases/assets/{asset_id}"
//         return self.get(param,self.headers)