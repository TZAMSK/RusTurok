#[cfg(test)]
mod tests {
    use crate::weapons::components::{PrimaryWeaponType, Stats, Weapon, WeaponTrait, WeaponType};

    #[test]
    fn create_a_hand_cannon() {
        let hand_cannon = Weapon::new(
            "a hand cannon".to_string(),
            WeaponType::PrimaryWeaponType(PrimaryWeaponType::HandCannon),
        );

        assert_eq!(
            hand_cannon,
            Weapon {
                name: "a hand cannon".to_string(),
                unique_trait: WeaponTrait {
                    bullet_speed: 100.0,
                    mag_size: 11.0,
                    stats: Stats {
                        range: 40.0,
                        stability: 40.0,
                        handling: 40.0,
                        reload: 30.0,
                        round_per_minute: 210.0,
                        aim_assist: 10.0,
                        zoom: 14.0
                    },
                    total_bullets: 120.0,
                    weapon_type: WeaponType::PrimaryWeaponType(PrimaryWeaponType::HandCannon)
                }
            }
        );
    }
}
