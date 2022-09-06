use clap::Parser;

pub mod minecraft {
    pub mod api;
    pub mod name;
    pub mod profile;
    pub mod propertie;
    pub mod uuid;
}

#[cfg(test)]
pub mod tests {
    #[cfg(test)]
    pub mod api;
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// minecraft的用户名
    #[clap(value_parser, default_value = "sagiri")]
    name: String,
}

#[tokio::main]
pub async fn main() {
    let args: Args = Args::parse();
    let uuid = crate::minecraft::api::get_uuid_by_name(&args.name).await;
    match uuid {
        Ok(uuid) => {
            println!("{}", uuid);
            let names = crate::minecraft::api::get_all_name_by_uuid(&uuid.id).await;

            match names {
                Ok(names) => {
                    for name in names {
                        println!("{}", name);
                    }
                }
                Err(error) => {
                    eprintln!("{}", error.to_string());
                }
            }
        }
        Err(err) => {
            eprintln!("{}", err.to_string())
        }
    }
}