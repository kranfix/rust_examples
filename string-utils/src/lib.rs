wai_bindgen_rust::export!("string-utils.wai");

pub struct StringUtils;

impl string_utils::StringUtils for StringUtils {
    fn is_empty(text: String) -> bool {
        text.is_empty()
    }
}

#[derive(Default)]
pub struct Request;

impl string_utils::Request for Request {
    fn new() -> wai_bindgen_rust::Handle<Request> {
        wai_bindgen_rust::Handle::new(Request)
    }

    fn headers(&self) -> Vec<String> {
        vec!["hola".to_string()]
    }
}
