use clap::Parser;
use fake::faker::lorem::en::Sentence;
use fake::faker::lorem::en::Word;
use fake::Faker;
use fake::{Dummy, Fake};

/// A bot to spam the hell out of the feed api.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// The url of the feed api.
    #[clap(short, long, default_value = "http://localhost:3000/message")]
    url: String,
    /// The time to wait between messages.
    #[clap(short, long, default_value_t = 1000)]
    interval_ms: u64,
}

struct Message {
    title: String,
    text: String,
}

impl Dummy<Faker> for Message {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &Faker, _rng: &mut R) -> Self {
        Message {
            title: Word().fake_with_rng(_rng),
            text: Sentence(1..3).fake_with_rng(_rng),
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let url = args.url;
    let interval_ms = args.interval_ms;
    let client = reqwest::Client::new();

    loop {
        let m = &Faker.fake::<Message>();

        let res = client
            .post(&url)
            .header("Content-Type", "application/json")
            .body(format!(r#"{{"title":"{}","text":"{}"}}"#, m.title, m.text))
            .send()
            .await
            .unwrap();

        std::thread::sleep(std::time::Duration::from_millis(interval_ms));
    }
}
