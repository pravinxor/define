///Public WordNik API Key found on the developer documentation
pub(crate) const API_KEY: &str = "c23b746d074135dc9500c0a61300a3cb7647e53ec2b9b658e";

///Removes quotes found at the start and end of a string
///
/// `assert!(clean_quotes(r#""hello""#), "hello")`
pub(crate) fn clean_quotes(str: &str) -> String {
    str.trim_start_matches('"').trim_end_matches('"').to_owned()
}

pub(crate) fn get(url: &str) {}
