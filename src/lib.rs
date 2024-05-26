use pyo3::prelude::*;
use std::collections::HashMap;
pub mod base_api;
pub mod salesforce_api;
use reqwest::blocking::{Client,Response};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn get_request(base_url: &str,header: HashMap<String,String>)-> PyResult<String>{
// fn get_request(base_url: &str,header: HashMap<String,String>)-> PyResult<HashMap<String,String>>{
    let res = match header.len(){
        0 => base_api::get_request(base_url,None),
        _ => {
            let constructed_header = base_api::construct_header(&header);
            base_api::get_request(base_url,constructed_header)
        },
    };
    // println!("{:#?}", &res.ok().unwrap());
    match res{
        Ok(result) => match result.text() {
            Ok(text) => Ok(text),
            Err(_) => Ok("Failed to read response text.".to_string()),
        },
        Err(_) => Ok("".to_string()),
    }
}


#[pyfunction]
fn get_sf_table(base_url: &str,username: &str,password: &str,table_name: &str,filter_query: &str )-> PyResult<String>{
    let result_string = salesforce_api::get_sf_table_main(base_url.to_string(), username.to_string() ,password.to_string() ,table_name.to_string() ,Some(filter_query.to_string()));
    // println!("{:?}",&result_string);
    match result_string {
        Some(result_str) => Ok(result_str),
        None => Ok("".to_string()),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn CoxRsPyo3Api(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(get_request, m)?)?;
    m.add_function(wrap_pyfunction!(get_sf_table, m)?)?;
    Ok(())
}
