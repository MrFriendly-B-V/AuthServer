use std::collections::HashMap;

pub fn hashmap_to_url(map: HashMap<&str, &str>) -> String {
    let mut url = "?".to_string();
    let mut index: usize = 1;
    for entry in &map {
        url += format!("{}={}", entry.0, entry.1).as_str();

        if index != map.len() {
            url += "&";
        }

        index += 1;
    }

    return url.to_string();
}