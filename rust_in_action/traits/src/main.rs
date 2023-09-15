use std::fmt::format;

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String{
        format!("Read more from {}...", self.summarize_author())
    }
}
pub struct BlogPost {
    pub title: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    author: String,
    username: String,
    message: String,
}

impl Summary for BlogPost {
    fn summarize_author(&self) -> String {
        format!("{} on his new blog post {}", self.author, self.title)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{} @{}", self.author, self.username)
    }
}

fn main() {
    let blog_post = BlogPost {
        title: String::from("My memoire"),
        author: String::from("Blanko Desarus"),
        content: String::from("I honestly had a good life with my puppy pinyato!")
    };

    let tweet = Tweet {
        author: String::from("Mitchy Degraseus"),
        username: String::from("thatgirlmitchy"),
        message: String::from("Once the goal is to win, you already lost")
    };

    println!("{}",tweet.summarize());

    println!("{} ",blog_post.summarize() );
}


