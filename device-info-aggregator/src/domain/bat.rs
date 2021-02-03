
use serde::{Serialize, Deserialize};
use std::fmt::Debug;
#[derive(Serialize, Deserialize, Debug)]
pub struct BatteryReport {
    device_id: String,
    percentage: f64,
    status: BatteryStatus
}
#[derive(Serialize, Deserialize, Debug)]
pub enum BatteryStatus {
    CHARGING,
    CHARGED,
    LOSING
}