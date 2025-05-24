pub enum ResourceType {
    Local,
    Remote,
    NotFound,
}

// If resource is a file on disk, return its contents; otherwise, treat it as a url and make request
pub fn get_resource_content(resource: &str) -> std::io::Result<String> {
    match resource_type(resource) {
        ResourceType::Local => std::fs::read_to_string(resource),
        ResourceType::Remote => read_web_to_string(resource),
        ResourceType::NotFound => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "resource not a file on disk and does not look like a url",
        )),
    }
}

fn resource_type(string: &str) -> ResourceType {
    match std::path::Path::new(string).is_file() {
        true => ResourceType::Local,
        false => {
            if string.starts_with("http") {
                ResourceType::Remote
            } else {
                ResourceType::NotFound
            }
        }
    }
}

fn read_web_to_string(url: &str) -> std::io::Result<String> {
    match reqwest::blocking::get(url) {
        Ok(response) => response.text().map_err(|err| {
            std::io::Error::other(format!("error converting response to text: {err}"))
        }),
        Err(err) => Err(std::io::Error::other(format!(
            "error fetching feed response: {err}"
        ))),
    }
}
