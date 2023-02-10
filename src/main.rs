use chrono::DateTime;
use clap::Parser;
use regex::Regex;
use reqwest::blocking::Client;
use rss::Channel;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// RSS url to fetch blogs from
   #[arg(short, long)]
   url: String,

   /// Number of times to greet
   #[arg(short, long, default_value_t = 5)]
   number_of_posts: u8,
}

fn main() {
    let args = Args::parse();
    let url = args.url;
    let number_of_posts = args.number_of_posts;

    let client = Client::new();
    let response = client.get(url).send().unwrap();
    let rss = response.bytes().unwrap();
    let channel = Channel::read_from(&rss[..]).unwrap();

    let mut content = String::new();
    for (_index, item) in channel.items().iter().enumerate().take(number_of_posts.into()) {
        let pub_date = DateTime::parse_from_rfc2822(item.pub_date().unwrap()).unwrap().format("%Y-%m-%d").to_string();
        content.push_str(&format!("[{}]({}) - {}\n\n", item.title().unwrap(), item.link().unwrap(), pub_date));
    }

    let re = Regex::new(r"(?s)<!-- blogs starts -->.*<!-- blogs ends -->").unwrap();
    let readme = std::fs::read_to_string("README.md").unwrap();
    let new_readme = re.replace_all(&readme, &format!("<!-- blogs starts -->\n{}\n<!-- blogs ends -->", content));
    std::fs::write("README.md", new_readme.as_bytes()).unwrap();
}
