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
                    mag_size: 11,
                    stats: Stats {
                        range: 40.0,
                        stability: 40.0,
                        handling: 40.0,
                        reload: 30.0,
                        round_per_minute: 210.0,
                        aim_assist: 10.0,
                        zoom: 14.0
                    },
                    total_bullets: 120,
                    weapon_type: WeaponType::PrimaryWeaponType(PrimaryWeaponType::HandCannon)
                }
            }
        );
    }

    #[test]
    fn create_an_auto_rifle() {
        let auto_rifle = Weapon::new(
            "an auto rifle".to_string(),
            WeaponType::PrimaryWeaponType(PrimaryWeaponType::AutoRifle),
        );

        assert_eq!(
            auto_rifle,
            Weapon {
                name: "an auto rifle".to_string(),
                unique_trait: WeaponTrait {
                    bullet_speed: 100.0,
                    mag_size: 32,
                    stats: Stats {
                        range: 20.0,
                        stability: 50.0,
                        handling: 50.0,
                        reload: 45.0,
                        round_per_minute: 600.0,
                        aim_assist: 10.0,
                        zoom: 14.0
                    },
                    total_bullets: 400,
                    weapon_type: WeaponType::PrimaryWeaponType(PrimaryWeaponType::AutoRifle)
                }
            }
        );
    }
}
