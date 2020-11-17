extern crate battery;

use serde::{Serialize};
use battery::units::ratio::percent;
use self::battery::units::energy::watt_hour;

#[derive(Serialize, Debug)]
pub struct BatteryInfo {
    state: String,
    ratio: f32,
    capacity_wh: f32,
    time_to_full: String
}

pub fn generate_battery_report() -> Result<Vec<BatteryInfo>, battery::Error> {
    let manager = battery::Manager::new()?;

    let mut batteries = Vec::new();

    for (_idx, maybe_battery) in manager.batteries()?.enumerate() {
        let battery = maybe_battery?;

        let bat = BatteryInfo {
            state: format!("{}", battery.state()),
            ratio: battery.state_of_charge().get::<percent>(),
            capacity_wh: battery.energy().get::<watt_hour>(),
            time_to_full: format!("{:?}", battery.time_to_full())
        };

        batteries.push(bat);
    }

    Ok(batteries)
}
