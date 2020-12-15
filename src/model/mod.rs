use crate::import::*;
use crate::util::datetime_util;
use rbatis::crud::CRUD;
use rbatis::core::Result as DbResult;

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
    #[serde(skip_serializing)]
    pub role: String,
    pub sex: char,
    #[serde(skip_serializing)]
    pub is_delete: u8,
    #[serde(serialize_with = "datetime_util::serialize_datetime")]
    pub create_time: NaiveDateTime,
    #[serde(serialize_with = "datetime_util::serialize_datetime")]
    pub update_time: NaiveDateTime,
}

impl User {
    pub async fn info(user_id:u32) -> DbResult<User>{
        DB.fetch_by_id("", &user_id).await
    }
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
    #[serde(skip_serializing)]
    pub is_delete: u8,
    #[serde(serialize_with = "datetime_util::serialize_datetime")]
    pub create_time: NaiveDateTime,
    #[serde(serialize_with = "datetime_util::serialize_datetime")]
    pub update_time: NaiveDateTime,
}

#[derive(CRUDEnable, Serialize, Deserialize, Clone, Debug)]
pub struct Activity {
    //表名称 Activity=> "activity"
    pub id: u32,
    #[serde(skip_serializing)]
    pub creator_id: u32,
    #[serde(skip_serializing)]
    pub last_editor_id: u32,
    #[serde(serialize_with = "datetime_util::serialize_datetime")]
    pub start_time: NaiveDateTime,
    #[serde(serialize_with = "datetime_util::serialize_datetime")]
    pub end_time: NaiveDateTime,
    pub subject: String,
    pub activity_type: String,
    pub apply_url: String,
    pub content: String,
    #[serde(skip_serializing)]
    pub is_delete: u8,
    #[serde(serialize_with = "datetime_util::serialize_datetime")]
    pub create_time: NaiveDateTime,
    #[serde(serialize_with = "datetime_util::serialize_datetime")]
    pub update_time: NaiveDateTime,
}

impl Activity {
  pub  async fn info(id: u32) -> Option<Activity> {
        let mut query = DB.new_wrapper();
        query
            .eq("id", id)
            .eq("is_delete", 0);
        query.check().map_or_else(|e| {
            None
        }, |w| block_on(async move {
            DB.fetch_by_wrapper::<Activity>("", &w)
                .await
                .map_or_else(|e| {
                    None
                }, |v| Some(v))
        }),
        )
    }

    pub  async fn exist(id: u32) -> bool{
        let sql = r#"
                SELECT id FROM activity
                WHERE is_delete = 0
                and id = #{id}"#;
        // DB.py_fetch("", sql,&json!({"id":id}))
        DB.py_fetch("", sql,&id)
            .await
            .unwrap_or(0) >= 1
    }
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


