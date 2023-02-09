use regex::Regex;
use reqwest::blocking::Client;
use rss::Channel;
use chrono::DateTime;

fn main() {
    let client = Client::new();
    let url = "https://blog.azanulhaque.tech/rss.xml";
    let response = client.get(url).send().unwrap();
    let rss = response.bytes().unwrap();
    let channel = Channel::read_from(&rss[..]).unwrap();

    let mut content = String::new();
    for (_index, item) in channel.items().iter().enumerate().take(5) {
        let pub_date = DateTime::parse_from_rfc2822(item.pub_date().unwrap()).unwrap().format("%Y-%m-%d").to_string();
        content.push_str(&format!("[{}]({}) - {}\n\n", item.title().unwrap(), item.link().unwrap(), pub_date));
    }

    let re = Regex::new(r"(?s)<!-- blogs starts -->.*<!-- blogs ends -->").unwrap();
    let readme = std::fs::read_to_string("README.md").unwrap();
    let new_readme = re.replace_all(&readme, &format!("<!-- blogs starts -->\n{}\n<!-- blogs ends -->", content));
    std::fs::write("README.md", new_readme.as_bytes()).unwrap();
}
