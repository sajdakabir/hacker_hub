trait Summary {
    fn get_username(&self)-> &str;
    fn summarize(&self)->String {
      format!(" {} (Read more...)", self.get_username())
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String
}
impl Summary for NewsArticle {
    fn get_username(&self)-> &str {
        self.author.as_str()
    }
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

impl Summary for Tweet {
    fn get_username(&self)-> &str {
        self.username.as_str()
    }
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

    new_aggregator(&news);
    new_aggregator(&tweet);
}

fn new_aggregator(source: &impl Summary){
    println!("the content {}", source.summarize());
}
