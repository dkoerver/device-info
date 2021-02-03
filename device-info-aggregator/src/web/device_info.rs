
use crate::info::bat_service;
use chrono::Utc;
use rocket_contrib::json::Json;

use crate::domain::bat::BatteryReport;
use crate::domain::bat::BatteryStatus;
use uuid::Uuid;

#[get("/device-info/batteries")]
pub fn bat() -> String {
    
    let result = bat_service::generate_battery_report();

    match result {
        Err(_e) => return "error".to_string(),
        Ok(vec) => return serde_json::to_string(&vec).unwrap(),
    }
}

#[put("/device-info/battery-stat/<deviceId>", format = "text/plain", data = "<report>")]
pub fn update_battery_report(deviceId: String, report: Json<BatteryReport>) {

    let rep = report.into_inner();

    println!("Report: {:?}", rep);    
}

#[get("/info/time")]
pub fn time() -> String {
    Utc::now().to_string()
}

