pub trait Summary {
	fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub trait Person {
    fn person_name(&self) -> String;
}

pub struct NewArticle {
	pub headline: String,
	pub location: String,
	pub author: String,
	pub content: String,
}

impl Summary for NewArticle  {
	fn summarize_author(&self) -> String {
		format!("{}, by {} ({})", self.headline, self.author, self.location)
	}
}

pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: bool,
	pub retweet: bool,
}

impl Summary for Tweet{
	fn summarize_author(&self) -> String {
		format!("@{}", self.username)
	}
}

impl Person for Tweet {
    fn person_name(&self) -> String {
        println!("user name is: {}", self.username);
        "ffff".to_string()        
    }
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn t_notify<T: Summary + Person>(item: T) {
    println!("{}", item.summarize_author());
    item.person_name();
}

fn main() {
    let tweet = Tweet{
        username: String::from("horse_ebook"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    //notify(tweet);

    t_notify(tweet);
}
