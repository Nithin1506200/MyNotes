use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::schema::merchant_account;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = merchant_account)]
pub struct MerchantAccountDb {
    pub id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub email: String,
    pub address: Value,          // JSONB Type
    pub allowed_payments: Value, // JSONB Type
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = merchant_account)]
pub struct CreateMerchantAccountDb {
    pub id: String,
    pub name: String,
    pub email: String,
    pub address: Value,          // JSONB Type
    pub allowed_payments: Value, // JSONB Type
    pub active: bool,
}
#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = merchant_account)]
pub struct UpdateMerchantAccountDb {
    name: Option<String>,
    pub email: Option<String>,
    pub address: Option<Value>,          // JSONB Type
    pub allowed_payments: Option<Value>, // JSONB Type
    pub active: Option<bool>,
}
