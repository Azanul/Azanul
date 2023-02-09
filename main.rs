use reqwest::Client;
use rss::Channel;

fn main() {
    let client = Client::new();
    let url = "https://www.example.com/rss";
    let response = client.get(url).send().unwrap();
    let rss = response.text().unwrap();
    let channel = Channel::from_xml(&rss).unwrap();

    let number_of_blogs = 5;
    println!("Latest {} blogs:", number_of_blogs);
    for (index, item) in channel.items().iter().enumerate().take(number_of_blogs) {
        println!("{}: {}", index + 1, item.title().unwrap());
    }
}
