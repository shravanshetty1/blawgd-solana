use chrono::prelude::Utc;
use tendermint::Time;
use tendermint_light_client::components::clock::Clock;

pub struct WasmClock;

impl Clock for WasmClock {
    fn now(&self) -> Time {
        Time::from(Utc::now())
    }
}
