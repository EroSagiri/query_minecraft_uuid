use clap::Parser;
use serde::{Serialize, Deserialize};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// minecraft的用户名
    #[clap(value_parser, default_value="sagiri")]
    name: String,
}

#[tokio::main]
async fn main() {
    let args : Args = Args::parse();
    let rest = reqwest::get("https://api.mojang.com/users/profiles/minecraft/".to_owned() + &args.name).await;
    match rest {
        Ok(rest) => {
            let uuid = rest.json::<Uuid>().await;

            match uuid {
                Ok(uuid) => {
                    println!("{}", uuid.to_string())
                }

                Err(e) => {
                    println!("{}", e.to_string())
                }
            }
        }
        Err(e) => {
            println!("{}", e.to_string())
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Uuid {
    name : String,
    id : String
}

impl Uuid {
    pub fn to_string(self) -> String {
        return format!("{name} {id}", name = self.name, id = self.id)
    }
}