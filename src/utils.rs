use reqwest::blocking::Client;
use serde_json::Value;

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
pub(crate) fn get(url: &str) -> Value {
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

///If a definition can be found, will return the definition at idx 0, and the attribution at idx 1
pub(crate) fn get_def(word: &Value) -> Result<(String, String), ()> {
    let definition = match word.get("text") {
        None => return Err(()),
        Some(definition) => definition
            .to_string()
            .replace("<xref>", "")
            .replace("</xref>", "")
            .replace("\\", ""),
    };

    let attribution_text = word.get("attributionText").unwrap().to_string();
    Ok((definition, attribution_text))
}

pub(crate) fn display_word(definition: &str, attribution: &str) {
    println!("{}\n\t-{}", definition, clean_quotes(attribution));
}
