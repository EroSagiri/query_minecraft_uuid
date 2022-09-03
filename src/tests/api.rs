#[tokio::test]
async fn test_get_uuid_by_name() {
    let uuid = crate::minecraft::api::get_uuid_by_name("ero_sagiri")
        .await;
    match uuid {
    Ok(uuid) => {
        println!("{}", uuid);
        assert!(true);
    },
    Err(error) => {
        eprint!("{}", error.to_string());
        assert!(false);
    },
}
}

#[tokio::test]
async fn test_get_all_name_by_uuid() {
    let names =
        crate::minecraft::api::get_all_name_by_uuid("f4dcda54d23b4a55b341d5538a85e077").await;

    match names {
        Ok(names) => {
            for name in names {
                println!("{}", name);
            }
        }
        Err(error) => {
            eprintln!("{}", error.to_string());
            assert!(false);
        }
    }
}

#[tokio::test]
async fn test_get_profile_by_uuid() {
    let profile = crate::minecraft::api::get_profile_by_uuid("f4dcda54d23b4a55b341d5538a85e077").await;

    match profile {
    Ok(profile) => {
        println!("{}", profile);
        for propertie in profile.properties {
            println!("{} {}", propertie.name, propertie.value);
        }
        assert!(true)
    },
    Err(error) => {
        eprintln!("{}", error.to_string());
        assert!(false)
    },
}
}