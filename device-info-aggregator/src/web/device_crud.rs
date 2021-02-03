
use crate::info::bat_service;
use chrono::Utc;
use rocket_contrib::json::Json;

use crate::domain::bat::BatteryReport;
use crate::domain::bat::BatteryStatus;
use uuid::Uuid;


#[post("/device-crud/register-device", format="text/plain")]
pub fn register_new_device() -> String {

    let result = Uuid::new_v4();

    result.to_simple().to_string()
}

#[delete("/device-crud/delete-device/<deviceId>")]
pub fn delete_device(deviceId: String) {

}