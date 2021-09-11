use reqwest::{blocking::Client, header::HeaderMap};

///Public WordNik API Key found on the developer documentation
pub(crate) const API_KEY: &str = "c23b746d074135dc9500c0a61300a3cb7647e53ec2b9b658e";

lazy_static::lazy_static! {
    static ref CLIENT: Client = Client::new();
}

///Removes quotes found at the start and end of a string
///
/// `assert!(clean_quotes(r#""hello""#), "hello")`
pub(crate) fn clean_quotes(str: &str) -> String {
    str.trim_start_matches('"').trim_end_matches('"').to_owned()
}
///Function to be used for interacting with WordNik Definitions API and returning a JSON of the Result
pub(crate) fn get(url: &str) -> serde_json::Value {
    CLIENT
        .get(url)
        .header("api_key", API_KEY)
        .header("limit", 1)
        .header("includeRelated", "false")
        .header("useCanonical", "false")
        .header("includeTags", "false")
        .send()
        .unwrap()
        .json()
        .unwrap()
}