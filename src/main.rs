use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main() {
    let article: Article = Article {
        article: String::from("How to work with JSON in Rust"),
        author: String::from("Danny"),
        paragraph: vec! [
            Paragraph {
                name: String::from("First Sentence")
            },
            Paragraph {
                name: String::from("Body of the paragraph")
            },
            Paragraph {
                name: String::from("End of the paragraph")
            }
        ]
    };
    let json = serde_json::to_string(&article).unwrap();
    println!("The JSON is: {}", json)
}