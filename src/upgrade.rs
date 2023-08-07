pub trait Upgrade {
    fn get_name(&self) -> &'static str;
}

pub struct SpeedUpgrade {}
pub struct FireRateUpgrade {}
pub struct RecoveryUpgrade {}

impl Upgrade for SpeedUpgrade {
    fn get_name(&self) -> &'static str {
        "Speed"
    }
}

impl Upgrade for FireRateUpgrade {
    fn get_name(&self) -> &'static str {
        "Fire Rate"
    }
}

impl Upgrade for RecoveryUpgrade {
    fn get_name(&self) -> &'static str {
        "Recovery"
    }
}

pub fn pick_random_upgrades() -> Vec<Box<dyn Upgrade>> {
    let mut upgrades : Vec<Box<dyn Upgrade>> = Vec::new();
    
    upgrades.push(Box::new(FireRateUpgrade{}));

    upgrades
}
