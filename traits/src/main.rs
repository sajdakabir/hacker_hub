
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool
}

fn main() {
    println!("Hello, world!");

    let tweet = Tweet {
        username: "Sajda".to_string(),
        content: "new post about march".to_string(),
        reply: false,
        retweet: false
    };

    new_aggregator(tweet);
}

fn new_aggregator(tweet: Tweet){
    println!("the content {}", tweet.content);
}
