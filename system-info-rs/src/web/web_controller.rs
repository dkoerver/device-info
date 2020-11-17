
use crate::info::bat;
use chrono::Utc;

#[get("/batteries")]
pub fn bat() -> String {
    
    let result = bat::generate_battery_report();

    match result {
        Err(_e) => return "error".to_string(),
        Ok(vec) => return serde_json::to_string(&vec).unwrap(),
    }
}

#[get("/time")]
pub fn time() -> String {
    Utc::now().to_string()
}

