trait Summary {
    fn summarize(&self)->String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String
}
impl Summary for NewsArticle {
    fn summarize(&self)->String {
        let summary = format!("the auhor {} and content {}", self.author, self.content);
        summary
    }
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

    let news = NewsArticle {
        headline: "what should i give".to_string(),
        location: "kolkata".to_string(),
        author: "sajda".to_string(),
        content: "she is the founder of march".to_string()
    };

    new_aggregator(news);
}

fn new_aggregator(source: impl Summary){
    println!("the content {}", source.summarize());
}
