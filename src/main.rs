use serde_derive::{Deserialize, Serialize};
use std::env;
use reqwest::{Url, StatusCode, blocking::Client};


#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseData {
    pub items: Vec<Item>,
    pub total: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    #[serde(rename = "id")]
    #[serde(alias = "uuid")]
    pub id: String,
}
fn get_delete_list(url: String, token: String) -> ResponseData{
    let req_url: Url = Url::parse(&url).expect("Error parsing");
    let client = Client::builder().danger_accept_invalid_certs(true).build().expect("Failed");
    let response = client
        .get(req_url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .expect("Request failed");
    if response.status()!=StatusCode::OK{
        println!("Request to get list failed with error code {}", response.status());
    }
    let tpares: ResponseData = serde_json::from_str(&response.text().expect("Parsing failed")).expect("Failed");
    tpares
}

fn delete_resource(url:String, token: String, delete_list: ResponseData){
    println!("Deletion Started...");
    println!("Total number of records to be deleted: {}", delete_list.total);
    for response in delete_list.items{
        let delete_url = format!("{}/{}",url, response.id);
        let req_url: Url = Url::parse(&delete_url).expect("Error parsing");
        println!("Delete URL {}", &req_url);
        let client = Client::builder().danger_accept_invalid_certs(true).build().expect("Failed");
        let delete_res = client.delete(req_url).header("Authorization", format!("Bearer {}", token)).send().expect("Failed");
        if delete_res.status() != StatusCode::OK{
            println!("Deletion failed for resource: {} with error {}",response.id, delete_res.status());
        }
    }
    println!("Deletion Done...");
}

fn main(){
    let api_url = match env::var("BASE_URL") { // Example https://server.apps.tpa.qe.net/api/v2/advisory
        Ok(url) => url,
        _ => panic!("ERROR: Specify BASE_URL env var (e.g. BASE_URL='http://localhost/api/v2/advisory')"),
    };
    let filter = match env::var("Q") { //q=modified%3E1996-12-31T18%3A30%3A00.000Z%26modified%3C2024-12-30T18%3A30%3A00.000Z
        Ok(query) => query,
        _ => panic!("ERROR: Specify in Q env var the query part of url (e.g. Q='q=modified...')"),
    };
    let token: String = match env::var("API_TOKEN") { //API Token
        Ok(token) => token,
        _ => panic!("ERROR: Specify authentication token for API (value for Bearer token, e.g. API_TOKEN=blah)"),
    };

    let get_url = format!("{}{}",api_url, filter);
    let res = get_delete_list(get_url.to_string(), token.to_string());
    delete_resource(api_url.to_string(), token.to_string(), res);

}

