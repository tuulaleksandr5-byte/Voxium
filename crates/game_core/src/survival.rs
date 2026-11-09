//! Независимые от ввода и рендера правила выживания.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameMode {
    #[default]
    Survival,
    Creative,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameRule {
    KeepInventory,
    NaturalRegeneration,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameRules {
    pub keep_inventory: bool,
    pub natural_regeneration: bool,
}

impl Default for GameRules {
    fn default() -> Self {
        Self {
            keep_inventory: false,
            natural_regeneration: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SurvivalConfig {
    pub max_health: f32,
    pub max_hunger: f32,
    pub max_saturation: f32,
    pub initial_saturation: f32,
    pub exhaustion_step: f32,
    pub sprint_per_meter: f32,
    pub jump_exhaustion: f32,
    pub sprint_jump_exhaustion: f32,
    pub damage_exhaustion: f32,
    pub mining_exhaustion: f32,
    pub attack_exhaustion: f32,
    pub regen_hunger: f32,
    pub regen_interval: f32,
    pub regen_amount: f32,
    pub regen_exhaustion: f32,
    pub starvation_interval: f32,
    pub starvation_damage: f32,
    pub starvation_floor: f32,
    pub air_seconds: f32,
    pub drowning_interval: f32,
    pub drowning_damage: f32,
    pub fall_safe_distance: f32,
    pub fall_damage_per_block: f32,
    pub lava_interval: f32,
    pub lava_damage: f32,
    pub fire_interval: f32,
    pub fire_damage: f32,
    pub burn_seconds: f32,
    pub void_interval: f32,
    pub void_damage: f32,
}

impl Default for SurvivalConfig {
    fn default() -> Self {
        Self::from_ron(include_str!("../../../assets/data/survival.ron")).unwrap_or(Self::FALLBACK)
    }
}

impl SurvivalConfig {
    // Резерв нужен только при ошибке во встроенном файле; внешние ошибки возвращаются вызывающему коду.
    const FALLBACK: Self = Self {
        max_health: 20.0,
        max_hunger: 20.0,
        max_saturation: 20.0,
        initial_saturation: 5.0,
        exhaustion_step: 4.0,
        sprint_per_meter: 0.1,
        jump_exhaustion: 0.05,
        sprint_jump_exhaustion: 0.2,
        damage_exhaustion: 0.1,
        mining_exhaustion: 0.005,
        attack_exhaustion: 0.1,
        regen_hunger: 18.0,
        regen_interval: 4.0,
        regen_amount: 1.0,
        regen_exhaustion: 6.0,
        starvation_interval: 4.0,
        starvation_damage: 1.0,
        starvation_floor: 2.0,
        air_seconds: 15.0,
        drowning_interval: 1.0,
        drowning_damage: 2.0,
        fall_safe_distance: 3.0,
        fall_damage_per_block: 1.0,
        lava_interval: 0.5,
        lava_damage: 4.0,
        fire_interval: 1.0,
        fire_damage: 1.0,
        burn_seconds: 8.0,
        void_interval: 1.0,
        void_damage: 4.0,
    };

    pub fn from_ron(source: &str) -> Result<Self, String> {
        let config: Self = ron::from_str(source)
            .map_err(|error| format!("Ошибка параметров выживания: {error}"))?;
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), String> {
        for (name, value, minimum, maximum) in [
            ("max_health", self.max_health, 1.0, 1000.0),
            ("max_hunger", self.max_hunger, 1.0, 1000.0),
            ("max_saturation", self.max_saturation, 0.0, 1000.0),
            (
                "initial_saturation",
                self.initial_saturation,
                0.0,
                self.max_saturation,
            ),
            ("exhaustion_step", self.exhaustion_step, 0.001, 1000.0),
            ("sprint_per_meter", self.sprint_per_meter, 0.0, 1000.0),
            ("jump_exhaustion", self.jump_exhaustion, 0.0, 1000.0),
            (
                "sprint_jump_exhaustion",
                self.sprint_jump_exhaustion,
                0.0,
                1000.0,
            ),
            ("damage_exhaustion", self.damage_exhaustion, 0.0, 1000.0),
            ("mining_exhaustion", self.mining_exhaustion, 0.0, 1000.0),
            ("attack_exhaustion", self.attack_exhaustion, 0.0, 1000.0),
            ("regen_hunger", self.regen_hunger, 0.001, self.max_hunger),
            ("regen_interval", self.regen_interval, 0.001, 3600.0),
            ("regen_amount", self.regen_amount, 0.001, self.max_health),
            ("regen_exhaustion", self.regen_exhaustion, 0.001, 1000.0),
            (
                "starvation_interval",
                self.starvation_interval,
                0.001,
                3600.0,
            ),
            ("starvation_damage", self.starvation_damage, 0.001, 1000.0),
            (
                "starvation_floor",
                self.starvation_floor,
                0.0,
                self.max_health,
            ),
            ("air_seconds", self.air_seconds, 0.001, 3600.0),
            ("drowning_interval", self.drowning_interval, 0.001, 3600.0),
            ("drowning_damage", self.drowning_damage, 0.001, 1000.0),
            ("fall_safe_distance", self.fall_safe_distance, 0.0, 1000.0),
            (
                "fall_damage_per_block",
                self.fall_damage_per_block,
                0.001,
                1000.0,
            ),
            ("lava_interval", self.lava_interval, 0.001, 3600.0),
            ("lava_damage", self.lava_damage, 0.001, 1000.0),
            ("fire_interval", self.fire_interval, 0.001, 3600.0),
            ("fire_damage", self.fire_damage, 0.001, 1000.0),
            ("burn_seconds", self.burn_seconds, 0.0, 3600.0),
            ("void_interval", self.void_interval, 0.001, 3600.0),
            ("void_damage", self.void_damage, 0.001, 1000.0),
        ] {
            if !value.is_finite() || value < minimum || value > maximum {
                return Err(format!(
                    "Параметр {name} должен быть конечным числом от {minimum} до {maximum}"
                ));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DamageSource {
    Fall,
    Drowning,
    Lava,
    Fire,
    Starvation,
    Void,
    Mob,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StateEvent {
    Damaged { source: DamageSource, amount: f32 },
    Healed { amount: f32 },
    Died { source: DamageSource },
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Environment {
    pub underwater: bool,
    pub in_lava: bool,
    pub in_fire: bool,
    pub out_of_world: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Vitals {
    pub health: f32,
    pub hunger: f32,
    pub saturation: f32,
    pub exhaustion: f32,
    pub air_seconds: f32,
    pub burning_seconds: f32,
    regen_elapsed: f64,
    starvation_elapsed: f64,
    drowning_elapsed: f64,
    lava_elapsed: f64,
    fire_elapsed: f64,
    void_elapsed: f64,
    dead_notified: bool,
}

impl Vitals {
    pub fn new(cfg: &SurvivalConfig) -> Self {
        let cfg = if cfg.validate().is_ok() {
            cfg
        } else {
            &SurvivalConfig::FALLBACK
        };
        Self {
            health: cfg.max_health,
            hunger: cfg.max_hunger,
            saturation: cfg.initial_saturation,
            exhaustion: 0.0,
            air_seconds: cfg.air_seconds,
            burning_seconds: 0.0,
            regen_elapsed: 0.0,
            starvation_elapsed: 0.0,
            drowning_elapsed: 0.0,
            lava_elapsed: 0.0,
            fire_elapsed: 0.0,
            void_elapsed: 0.0,
            dead_notified: false,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.health <= 0.0 || self.dead_notified
    }

    pub fn damage(
        &mut self,
        amount: f32,
        source: DamageSource,
        mode: GameMode,
        cfg: &SurvivalConfig,
    ) -> Vec<StateEvent> {
        if mode == GameMode::Creative
            || self.is_dead()
            || !amount.is_finite()
            || amount <= 0.0
            || cfg.validate().is_err()
            || !self.finite()
        {
            return Vec::new();
        }
        let available = if source == DamageSource::Starvation {
            (self.health - cfg.starvation_floor).max(0.0)
        } else {
            self.health
        };
        let actual = amount.min(available);
        if actual <= 0.0 {
            return Vec::new();
        }
        self.exhaust(cfg.damage_exhaustion, mode, cfg);
        self.health = (self.health - actual).max(0.0);
        let mut events = vec![StateEvent::Damaged {
            source,
            amount: actual,
        }];
        if self.is_dead() && !self.dead_notified {
            self.dead_notified = true;
            events.push(StateEvent::Died { source });
        }
        events
    }

    pub fn exhaust(&mut self, amount: f32, mode: GameMode, cfg: &SurvivalConfig) {
        if mode == GameMode::Creative
            || self.is_dead()
            || !amount.is_finite()
            || amount <= 0.0
            || cfg.validate().is_err()
            || !self.finite()
        {
            return;
        }
        // f64 позволяет обработать большой конечный расход без переполнения и длинного цикла.
        let total = f64::from(self.exhaustion) + f64::from(amount);
        let step = f64::from(cfg.exhaustion_step);
        let units = (total / step).floor();
        let saturation_units = units.min(f64::from(self.saturation.ceil()));
        self.saturation = (f64::from(self.saturation) - saturation_units).max(0.0) as f32;
        self.hunger = (f64::from(self.hunger) - (units - saturation_units)).max(0.0) as f32;
        self.exhaustion = (total % step) as f32;
    }

    pub fn tick(
        &mut self,
        dt: f32,
        environment: Environment,
        mode: GameMode,
        rules: &GameRules,
        cfg: &SurvivalConfig,
    ) -> Vec<StateEvent> {
        if mode == GameMode::Creative
            || self.is_dead()
            || !dt.is_finite()
            || dt <= 0.0
            || cfg.validate().is_err()
            || !self.finite()
        {
            return Vec::new();
        }
        self.health = self.health.min(cfg.max_health);
        self.hunger = self.hunger.min(cfg.max_hunger);
        self.saturation = self.saturation.min(cfg.max_saturation);
        self.air_seconds = self.air_seconds.min(cfg.air_seconds);
        if !environment.underwater {
            self.air_seconds = cfg.air_seconds;
            self.drowning_elapsed = 0.0;
        }
        let contact = environment.in_lava || environment.in_fire;
        if contact {
            self.burning_seconds = cfg.burn_seconds;
        }

        let mut events = Vec::new();
        let mut remaining = f64::from(dt);
        const EPSILON: f64 = 1.0e-7;
        // Разбиваем кадр по ближайшему событию: воздух и горение не действуют задним числом.
        while remaining > 0.0 && !self.is_dead() {
            let regeneration = rules.natural_regeneration
                && self.hunger >= cfg.regen_hunger
                && self.health < cfg.max_health;
            let starvation = self.hunger <= 0.0 && self.health > cfg.starvation_floor;
            let drowning = environment.underwater && self.air_seconds <= 0.0;
            let lava = environment.in_lava;
            let fire = !lava && (environment.in_fire || self.burning_seconds > 0.0);
            let void = environment.out_of_world;
            let mut step = remaining;
            for (active, elapsed, interval) in [
                (regeneration, &mut self.regen_elapsed, cfg.regen_interval),
                (
                    starvation,
                    &mut self.starvation_elapsed,
                    cfg.starvation_interval,
                ),
                (drowning, &mut self.drowning_elapsed, cfg.drowning_interval),
                (lava, &mut self.lava_elapsed, cfg.lava_interval),
                (fire, &mut self.fire_elapsed, cfg.fire_interval),
                (void, &mut self.void_elapsed, cfg.void_interval),
            ] {
                if active {
                    step = step.min((f64::from(interval) - *elapsed).max(0.0));
                } else {
                    *elapsed = 0.0;
                }
            }
            if environment.underwater && self.air_seconds > 0.0 {
                step = step.min(f64::from(self.air_seconds));
            }
            if !contact && self.burning_seconds > 0.0 {
                step = step.min(f64::from(self.burning_seconds));
            }
            for (active, elapsed) in [
                (regeneration, &mut self.regen_elapsed),
                (starvation, &mut self.starvation_elapsed),
                (drowning, &mut self.drowning_elapsed),
                (lava, &mut self.lava_elapsed),
                (fire, &mut self.fire_elapsed),
                (void, &mut self.void_elapsed),
            ] {
                if active {
                    *elapsed += step;
                }
            }
            if environment.underwater {
                self.air_seconds = (f64::from(self.air_seconds) - step).max(0.0) as f32;
            }
            if !contact {
                self.burning_seconds = (f64::from(self.burning_seconds) - step).max(0.0) as f32;
            }
            remaining = (remaining - step).max(0.0);

            let mut pending_damage = Vec::new();
            for (active, source, elapsed, interval, amount) in [
                (
                    drowning,
                    DamageSource::Drowning,
                    &mut self.drowning_elapsed,
                    cfg.drowning_interval,
                    cfg.drowning_damage,
                ),
                (
                    lava,
                    DamageSource::Lava,
                    &mut self.lava_elapsed,
                    cfg.lava_interval,
                    cfg.lava_damage,
                ),
                (
                    fire,
                    DamageSource::Fire,
                    &mut self.fire_elapsed,
                    cfg.fire_interval,
                    cfg.fire_damage,
                ),
                (
                    void,
                    DamageSource::Void,
                    &mut self.void_elapsed,
                    cfg.void_interval,
                    cfg.void_damage,
                ),
                (
                    starvation,
                    DamageSource::Starvation,
                    &mut self.starvation_elapsed,
                    cfg.starvation_interval,
                    cfg.starvation_damage,
                ),
            ] {
                if active && *elapsed + EPSILON >= f64::from(interval) {
                    *elapsed = (*elapsed - f64::from(interval)).max(0.0);
                    // Применяем после освобождения ссылки на таймер ниже.
                    pending_damage.push((source, amount));
                }
            }
            // Применяем только новые отметки этого шага, не повторяя уже выданный урон.
            for (source, amount) in pending_damage {
                events.extend(self.damage(amount, source, mode, cfg));
            }
            if regeneration && self.regen_elapsed + EPSILON >= f64::from(cfg.regen_interval) {
                self.regen_elapsed = (self.regen_elapsed - f64::from(cfg.regen_interval)).max(0.0);
                if !self.is_dead() && self.hunger >= cfg.regen_hunger {
                    let amount = cfg.regen_amount.min(cfg.max_health - self.health);
                    if amount > 0.0 {
                        self.health += amount;
                        self.exhaust(cfg.regen_exhaustion, mode, cfg);
                        events.push(StateEvent::Healed { amount });
                    }
                }
            }
        }
        events
    }

    pub fn respawn(&mut self, cfg: &SurvivalConfig) {
        *self = Self::new(cfg);
    }

    fn finite(&self) -> bool {
        [
            self.health,
            self.hunger,
            self.saturation,
            self.exhaustion,
            self.air_seconds,
            self.burning_seconds,
        ]
        .into_iter()
        .all(|value| value.is_finite() && value >= 0.0)
    }
}

pub fn fall_damage(distance: f32, cfg: &SurvivalConfig) -> f32 {
    if !distance.is_finite() || distance <= 0.0 || cfg.validate().is_err() {
        return 0.0;
    }
    let blocks = f64::from((distance - cfg.fall_safe_distance).max(0.0).floor());
    (blocks * f64::from(cfg.fall_damage_per_block)).min(f64::from(f32::MAX)) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tick(vitals: &mut Vitals, seconds: f32, environment: Environment) -> Vec<StateEvent> {
        vitals.tick(
            seconds,
            environment,
            GameMode::Survival,
            &GameRules::default(),
            &SurvivalConfig::default(),
        )
    }

    #[test]
    fn creative_ignores_damage_exhaustion_and_environment() {
        let cfg = SurvivalConfig::default();
        let mut vitals = Vitals::new(&cfg);
        let before = vitals.clone();
        assert!(
            vitals
                .damage(100.0, DamageSource::Void, GameMode::Creative, &cfg)
                .is_empty()
        );
        vitals.exhaust(100.0, GameMode::Creative, &cfg);
        let environment = Environment {
            underwater: true,
            in_lava: true,
            in_fire: true,
            out_of_world: true,
        };
        assert!(
            vitals
                .tick(
                    100.0,
                    environment,
                    GameMode::Creative,
                    &GameRules::default(),
                    &cfg
                )
                .is_empty()
        );
        assert_eq!(vitals, before);
    }

    #[test]
    fn exhaustion_consumes_saturation_before_hunger_at_exact_boundary() {
        let cfg = SurvivalConfig::default();
        let mut vitals = Vitals::new(&cfg);
        vitals.saturation = 1.0;
        vitals.exhaust(3.5, GameMode::Survival, &cfg);
        assert_eq!(vitals.saturation, 1.0);
        vitals.exhaust(0.5, GameMode::Survival, &cfg);
        assert_eq!(
            (vitals.saturation, vitals.hunger, vitals.exhaustion),
            (0.0, 20.0, 0.0)
        );
        vitals.exhaust(8.5, GameMode::Survival, &cfg);
        assert_eq!(
            (vitals.saturation, vitals.hunger, vitals.exhaustion),
            (0.0, 18.0, 0.5)
        );
    }

    #[test]
    fn regeneration_spends_saturation_and_respects_hunger_and_rule() {
        let cfg = SurvivalConfig::default();
        let mut vitals = Vitals::new(&cfg);
        vitals.health = 18.0;
        assert!(tick(&mut vitals, 3.0, Environment::default()).is_empty());
        assert_eq!(
            tick(&mut vitals, 1.0, Environment::default()),
            vec![StateEvent::Healed { amount: 1.0 }]
        );
        assert_eq!(
            (vitals.health, vitals.saturation, vitals.exhaustion),
            (19.0, 4.0, 2.0)
        );
        tick(&mut vitals, 4.0, Environment::default());
        assert_eq!(
            (vitals.health, vitals.saturation, vitals.exhaustion),
            (20.0, 2.0, 0.0)
        );

        vitals.health = 18.0;
        vitals.hunger = 17.0;
        assert!(tick(&mut vitals, 8.0, Environment::default()).is_empty());
        vitals.hunger = 20.0;
        let rules = GameRules {
            natural_regeneration: false,
            ..GameRules::default()
        };
        assert!(
            vitals
                .tick(
                    8.0,
                    Environment::default(),
                    GameMode::Survival,
                    &rules,
                    &cfg
                )
                .is_empty()
        );
        assert_eq!(vitals.health, 18.0);
    }

    #[test]
    fn death_fires_once_and_dead_stats_wait_for_respawn() {
        let cfg = SurvivalConfig::default();
        let mut vitals = Vitals::new(&cfg);
        let events = vitals.damage(100.0, DamageSource::Mob, GameMode::Survival, &cfg);
        assert_eq!(
            events,
            vec![
                StateEvent::Damaged {
                    source: DamageSource::Mob,
                    amount: 20.0
                },
                StateEvent::Died {
                    source: DamageSource::Mob
                }
            ]
        );
        let dead = vitals.clone();
        assert!(
            vitals
                .damage(1.0, DamageSource::Fire, GameMode::Survival, &cfg)
                .is_empty()
        );
        assert!(tick(&mut vitals, 100.0, Environment::default()).is_empty());
        vitals.exhaust(20.0, GameMode::Survival, &cfg);
        assert_eq!(vitals, dead);
        vitals.respawn(&cfg);
        assert_eq!(
            (
                vitals.health,
                vitals.hunger,
                vitals.saturation,
                vitals.air_seconds
            ),
            (20.0, 20.0, 5.0, 15.0)
        );
        assert!(!vitals.is_dead());
        assert_eq!(
            vitals
                .damage(20.0, DamageSource::Fall, GameMode::Survival, &cfg)
                .iter()
                .filter(|event| matches!(event, StateEvent::Died { .. }))
                .count(),
            1
        );
    }

    #[test]
    fn drowning_starts_only_after_air_then_full_damage_interval() {
        let mut vitals = Vitals::new(&SurvivalConfig::default());
        let underwater = Environment {
            underwater: true,
            ..Environment::default()
        };
        assert!(tick(&mut vitals, 15.0, underwater).is_empty());
        assert_eq!((vitals.air_seconds, vitals.health), (0.0, 20.0));
        assert!(tick(&mut vitals, 0.5, underwater).is_empty());
        assert_eq!(
            tick(&mut vitals, 0.5, underwater),
            vec![StateEvent::Damaged {
                source: DamageSource::Drowning,
                amount: 2.0
            }]
        );
        assert_eq!(vitals.health, 18.0);

        let mut one_frame = Vitals::new(&SurvivalConfig::default());
        tick(&mut one_frame, 17.0, underwater);
        assert_eq!((one_frame.health, one_frame.air_seconds), (16.0, 0.0));
        tick(&mut one_frame, 0.1, Environment::default());
        assert_eq!(one_frame.air_seconds, 15.0);
    }

    #[test]
    fn starvation_stops_at_one_heart_and_fall_counts_complete_blocks() {
        let cfg = SurvivalConfig::default();
        let mut vitals = Vitals::new(&cfg);
        vitals.hunger = 0.0;
        vitals.saturation = 0.0;
        assert!(tick(&mut vitals, 3.9, Environment::default()).is_empty());
        tick(&mut vitals, 0.1, Environment::default());
        assert_eq!(vitals.health, 19.0);
        tick(&mut vitals, 1000.0, Environment::default());
        assert_eq!(vitals.health, 2.0);
        assert!(!vitals.is_dead());
        assert_eq!(fall_damage(3.0, &cfg), 0.0);
        assert_eq!(fall_damage(3.9, &cfg), 0.0);
        assert_eq!(fall_damage(4.0, &cfg), 1.0);
        assert_eq!(fall_damage(7.8, &cfg), 4.0);
        assert_eq!(fall_damage(f32::NAN, &cfg), 0.0);
    }

    #[test]
    fn environment_damage_burns_after_exit_and_void_kills_once() {
        let cfg = SurvivalConfig::default();
        let mut vitals = Vitals::new(&cfg);
        let lava = Environment {
            in_lava: true,
            ..Environment::default()
        };
        assert!(tick(&mut vitals, 0.25, lava).is_empty());
        assert_eq!(
            tick(&mut vitals, 0.25, lava),
            vec![StateEvent::Damaged {
                source: DamageSource::Lava,
                amount: 4.0
            }]
        );
        assert_eq!(vitals.burning_seconds, 8.0);
        let events = tick(&mut vitals, 1.0, Environment::default());
        assert!(events.contains(&StateEvent::Damaged {
            source: DamageSource::Fire,
            amount: 1.0
        }));
        assert_eq!(vitals.burning_seconds, 7.0);
        let void = Environment {
            out_of_world: true,
            ..Environment::default()
        };
        let events = tick(&mut vitals, 100.0, void);
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(event, StateEvent::Died { .. }))
                .count(),
            1
        );
    }

    #[test]
    fn invalid_time_damage_and_exhaustion_do_not_corrupt_state() {
        let cfg = SurvivalConfig::default();
        let mut vitals = Vitals::new(&cfg);
        let before = vitals.clone();
        for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, -1.0, 0.0] {
            assert!(tick(&mut vitals, invalid, Environment::default()).is_empty());
            assert!(
                vitals
                    .damage(invalid, DamageSource::Fall, GameMode::Survival, &cfg)
                    .is_empty()
            );
            vitals.exhaust(invalid, GameMode::Survival, &cfg);
        }
        assert_eq!(vitals, before);
    }

    #[test]
    fn configuration_rejects_unknown_fields_non_finite_and_invalid_boundaries() {
        let source = include_str!("../../../assets/data/survival.ron");
        assert!(SurvivalConfig::from_ron(source).is_ok());
        assert!(
            SurvivalConfig::from_ron(&source.replace("max_health: 20.0", "max_health: NaN"))
                .is_err()
        );
        assert!(
            SurvivalConfig::from_ron(&source.replace("regen_interval: 4.0", "regen_interval: 0.0"))
                .is_err()
        );
        assert!(
            SurvivalConfig::from_ron(
                &source.replace("initial_saturation: 5.0", "initial_saturation: 21.0")
            )
            .is_err()
        );
        assert!(SurvivalConfig::from_ron(&source.replacen('(', "(unknown: 1,", 1)).is_err());
        assert!(SurvivalConfig::from_ron("()").is_err());
    }

    #[test]
    fn long_frames_do_not_repeat_previous_damage_or_skip_damage_intervals() {
        let cfg = SurvivalConfig::default();
        let rules = GameRules {
            natural_regeneration: false,
            ..GameRules::default()
        };
        let environment = Environment {
            underwater: true,
            ..Environment::default()
        };
        let mut batched = Vitals::new(&cfg);
        let events = batched.tick(18., environment, GameMode::Survival, &rules, &cfg);
        assert_eq!(events.len(), 3);
        assert_eq!(batched.health, 14.);
        let mut small_steps = Vitals::new(&cfg);
        for _ in 0..72 {
            small_steps.tick(0.25, environment, GameMode::Survival, &rules, &cfg);
        }
        assert_eq!(batched, small_steps);
    }
}
