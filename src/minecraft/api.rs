use reqwest::Error;
use crate::minecraft::uuid::Uuid;

use super::{profile::Profile, name::Name};

///通过用户名获取用户uuid
pub(crate) async fn get_uuid_by_name(name: &str) -> Result<Uuid, Error> {
    let rest = reqwest::get(format!("https://api.mojang.com/users/profiles/minecraft/{name}", name = name)).await.unwrap();

    let uuid = rest.json::<Uuid>().await;
    return uuid
}

///通过uuid得到用户信息
pub async fn get_profile_by_uuid(uuid: &str) -> Result<Profile, Error> {
    let rest = reqwest::get(format!("https://sessionserver.mojang.com/session/minecraft/profile/{uuid}", uuid = uuid)).await.unwrap();

    let profile = rest.json::<Profile>().await;
    profile
}

/// 用过uuid得到使用过的名字
/// 为什么这个接口返回的json是不标准的吖
pub async fn get_all_name_by_uuid(uuid: &str) -> Result<Vec<Name>, Error> {
    let url = format!("https://api.mojang.com/user/profiles/{}/names", uuid = uuid);
    let rest = reqwest::get(url).await.unwrap();

    let profile = rest.json::<Vec<Name>>().await;
    // 这个接口返回非标准的json格式，无法直接进行反序列化,太难啦
    todo!();
    profile
}