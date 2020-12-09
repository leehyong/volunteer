use crate::import::*;

#[derive(CRUDEnable, Serialize, Deserialize, Clone, Debug)]
pub struct User {
    //表名称 User=> "user"
    pub id: u32,
    pub name: String,
    pub parent_id: Option<u32>,
    pub ancestor_id: Option<u32>,
    pub depth: u8,
    pub mobile: String,
    pub lang: String,
    pub country_code: String,
    pub role: String,
    pub sex: char,
    pub is_delete: u8,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

#[derive(CRUDEnable, Serialize, Deserialize, Clone, Debug)]
pub struct ThirdPartyUser {
    //表名称 ThirdPartyUser=> "third_party_user"
    pub id: u32,
    pub name: String,
    pub user_id: u32,
    pub account: String,
    pub token: String,
    pub source: String,
    pub is_delete: u8,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

#[derive(CRUDEnable, Serialize, Deserialize, Clone, Debug)]
pub struct Activity {
    //表名称 Activity=> "activity"
    pub id: u32,
    pub creator_id: u32,
    pub last_editor_id: u32,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub subject: String,
    pub activity_type: String,
    pub apply_url: String,
    pub content: String,
    pub is_delete: u8,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

#[derive(CRUDEnable, Serialize, Deserialize, Clone, Debug)]
pub struct Apply {
    //表名称 Apply=> "apply"
    pub id: u32,
    pub user_id: u32,
    pub activity_id: u32,
    pub is_delete: u8,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}


