use clap::Parser;

pub mod minecraft {
    pub mod api;
    pub mod profile;
    pub mod propertie;
    pub mod uuid;
    pub mod name;
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// minecraft的用户名
    #[clap(value_parser, default_value = "sagiri")]
    name: String,
}

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();
    let uuid = crate::minecraft::api::get_uuid_by_name(&args.name).await;
    match uuid {
        Ok(uuid) => {
            println!("{}", uuid)
        }
        Err(err) => {
            eprintln!("{}", err.to_string())
        }
    }
}