use crate::common::ApiError;
use crate::database::PgPool;
use crate::db_models::merchant_account_db::{
    CreateMerchantAccountDb, MerchantAccountDb, UpdateMerchantAccountDb,
};
use crate::schema::merchant_account::dsl::merchant_account;
use anyhow::Ok;
use diesel::{Insertable, RunQueryDsl};
use serde_json::Value;
use ulid::Ulid;
pub async fn create_new_merchant(
    conn: &mut PgPool,
    id: Option<&str>,
    name: String,
    email: String,
    address: Value,
    allowed_payments: Value,
    active: bool,
) -> Result<MerchantAccountDb, ApiError> {
    let merchant_account_: CreateMerchantAccountDb = CreateMerchantAccountDb {
        id: id.map_or(Ulid::new().to_string(), |e| e.to_string()),
        name,
        email,
        address,
        allowed_payments,
        active,
    };
    let mut conn = conn.get().expect("fsad");
    merchant_account_
        .insert_into(merchant_account)
        .get_result(&mut conn)
        .map_err(|e| ApiError::DatabaseError)
}
/// get only active merchant
pub fn get_merchant_by_id(id: &str) {
    todo!()
}
pub fn unsafe_get_merchant_by_id() {
    todo!()
}
pub fn update_merchant(merchant: &UpdateMerchantAccountDb) -> Result<(), ()> {
    todo!()
}
