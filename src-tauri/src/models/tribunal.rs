use crate::models::establish_connection;
use crate::schema::tribunals;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize, AsChangeset, Deserialize)]
#[diesel(belongs_to(AppealCourt))]
#[diesel(belongs_to(Group))]
#[serde(rename_all = "camelCase")]
pub struct Tribunal {
    pub id: i32,
    pub name: String,
    pub color: String,
    pub time_to: Option<i32>,
    pub group_id: i32,
    pub appeal_court_id: i32,
}

pub fn tribunal_list() -> Vec<Tribunal> {
    tribunals::dsl::tribunals
        .select(tribunals::all_columns)
        .load::<Tribunal>(&mut establish_connection())
        .expect("Loading tribunals failed")
}

pub fn tribunal_update(tribunal: Tribunal) -> Tribunal {
    diesel::update(tribunals::table.find(tribunal.id))
        .set(&tribunal)
        .get_result(&mut establish_connection())
        .unwrap()
}
