use crate::survival_client::PlayerAction;
use crate::{
    session::{Game, Mode},
    survival_client::PlayerActionMessage,
};
use bevy::{
    input::{ButtonState, keyboard::KeyboardInput},
    prelude::*,
};
use game_core::survival::{GameMode, GameRule};

#[derive(Resource, Default)]
pub struct ConsoleState {
    pub input: String,
    pub feedback: String,
}

pub fn plugin(app: &mut App) {
    app.init_resource::<ConsoleState>().add_systems(
        Update,
        keyboard_input
            .after(crate::player::controls)
            .before(crate::survival_client::apply_actions),
    );
}

fn keyboard_input(
    mut keyboard: MessageReader<KeyboardInput>,
    mut game: ResMut<Game>,
    mut console: ResMut<ConsoleState>,
    mut actions: MessageWriter<PlayerActionMessage>,
) {
    for event in keyboard.read() {
        if event.state != ButtonState::Pressed {
            continue;
        }
        if game.mode == Mode::Playing
            && game.world_id.is_some()
            && event.key_code == KeyCode::Slash
            && !event.repeat
        {
            console.input = "/".into();
            console.feedback.clear();
            game.screen(Mode::Console);
            continue;
        }
        if game.mode != Mode::Console {
            continue;
        }
        match event.key_code {
            KeyCode::Escape => {
                console.input.clear();
                console.feedback.clear();
                game.screen(Mode::Playing);
            }
            KeyCode::Enter | KeyCode::NumpadEnter if !event.repeat => {
                match parse_command(&console.input) {
                    Ok(action) => {
                        actions.write(PlayerActionMessage(action));
                        console.input.clear();
                        console.feedback.clear();
                        game.screen(Mode::Playing);
                    }
                    Err(error) => {
                        console.feedback = error;
                        game.ui_dirty = true;
                    }
                }
            }
            KeyCode::Backspace => {
                console.input.pop();
                game.ui_dirty = true;
            }
            _ => {
                if let Some(text) = &event.text {
                    let remaining = 256_usize.saturating_sub(console.input.chars().count());
                    console.input.extend(
                        text.chars()
                            .filter(|c| !c.is_control() && !matches!(c, '\u{2028}' | '\u{2029}'))
                            .take(remaining),
                    );
                    game.ui_dirty = true;
                }
            }
        }
    }
}

pub fn parse_command(text: &str) -> Result<PlayerAction, String> {
    let mut arguments = text.split_whitespace();
    match arguments.next() {
        Some("/gamemode") => {
            let mode = arguments.next();
            if mode.is_none() || arguments.next().is_some() {
                return Err("Использование: /gamemode survival|creative|0|1".into());
            }
            match mode {
                Some("survival" | "0") => Ok(PlayerAction::SetMode(GameMode::Survival)),
                Some("creative" | "1") => Ok(PlayerAction::SetMode(GameMode::Creative)),
                _ => Err("Неизвестный режим: используйте survival, creative, 0 или 1".into()),
            }
        }
        Some("/gamerule") => {
            let rule = arguments.next();
            let value = arguments.next();
            if rule.is_none() || value.is_none() || arguments.next().is_some() {
                return Err(
                    "Использование: /gamerule keepInventory|naturalRegeneration true|false".into(),
                );
            }
            let rule = match rule {
                Some("keepInventory") => GameRule::KeepInventory,
                Some("naturalRegeneration") => GameRule::NaturalRegeneration,
                _ => {
                    return Err("Неизвестное правило: keepInventory или naturalRegeneration".into());
                }
            };
            let value = match value {
                Some("true") => true,
                Some("false") => false,
                _ => return Err("Значение правила должно быть true или false".into()),
            };
            Ok(PlayerAction::SetRule(rule, value))
        }
        Some("/summon") => {
            let (Some(name), None) = (arguments.next(), arguments.next()) else {
                return Err("Использование: /summon cow|pig|sheep|chicken|ghoul|spider".into());
            };
            game_core::mobs::MobKind::ALL
                .into_iter()
                .find(|kind| kind.spec().id == name)
                .map(PlayerAction::Summon)
                .ok_or_else(|| format!("Неизвестный моб: {name}"))
        }
        Some("/time") => match (arguments.next(), arguments.next()) {
            (Some("day"), None) => Ok(PlayerAction::SetDay(true)),
            (Some("night"), None) => Ok(PlayerAction::SetDay(false)),
            _ => Err("Использование: /time day|night".into()),
        },
        Some(command) => Err(format!("Неизвестная команда: {command}")),
        None => Err("Введите команду: /gamemode, /gamerule, /summon или /time".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input_app(mode: Mode, has_world: bool) -> App {
        let mut app = App::new();
        app.init_resource::<Game>()
            .init_resource::<ConsoleState>()
            .add_message::<KeyboardInput>()
            .add_message::<PlayerActionMessage>()
            .add_systems(Update, keyboard_input);
        let mut game = app.world_mut().resource_mut::<Game>();
        game.mode = mode;
        game.world_id = has_world.then(|| "test-world".into());
        game.ui_dirty = false;
        app
    }

    fn press(app: &mut App, key_code: KeyCode, text: Option<&str>) {
        app.world_mut().write_message(KeyboardInput {
            key_code,
            logical_key: bevy::input::keyboard::Key::Character(text.unwrap_or("").into()),
            state: ButtonState::Pressed,
            text: text.map(Into::into),
            repeat: false,
            window: Entity::PLACEHOLDER,
        });
    }

    #[test]
    fn parses_named_and_numeric_game_modes() {
        for (command, expected) in [
            ("/gamemode survival", GameMode::Survival),
            ("/gamemode 0", GameMode::Survival),
            ("/gamemode creative", GameMode::Creative),
            ("/gamemode 1", GameMode::Creative),
            ("  /gamemode   survival  ", GameMode::Survival),
        ] {
            assert!(
                matches!(parse_command(command), Ok(PlayerAction::SetMode(mode)) if mode == expected),
                "Команда: {command}"
            );
        }
    }

    #[test]
    fn parses_both_game_rules_and_boolean_values() {
        for (command, expected_rule, expected_value) in [
            (
                "/gamerule keepInventory true",
                GameRule::KeepInventory,
                true,
            ),
            (
                "/gamerule keepInventory false",
                GameRule::KeepInventory,
                false,
            ),
            (
                "/gamerule naturalRegeneration true",
                GameRule::NaturalRegeneration,
                true,
            ),
            (
                "/gamerule naturalRegeneration false",
                GameRule::NaturalRegeneration,
                false,
            ),
        ] {
            assert!(
                matches!(parse_command(command), Ok(PlayerAction::SetRule(rule, value)) if rule == expected_rule && value == expected_value),
                "Команда: {command}"
            );
        }
    }

    #[test]
    fn rejects_unknown_commands_missing_arguments_extra_arguments_and_bad_values() {
        for command in [
            "",
            "/",
            "gamemode survival",
            "/unknown",
            "/respawn",
            "/gamemode",
            "/gamemode survival extra",
            "/gamemode 2",
            "/gamemode spectator",
            "/gamerule",
            "/gamerule keepInventory",
            "/gamerule keepInventory true extra",
            "/gamerule unknown true",
            "/gamerule keepInventory 1",
            "/gamerule keepInventory TRUE",
            "/gamerule naturalRegeneration on",
        ] {
            let error = parse_command(command).expect_err(command);
            assert!(!error.is_empty(), "Команда: {command}");
            assert!(error.chars().any(|ch| ('А'..='я').contains(&ch)));
        }
    }

    #[test]
    fn opening_console_consumes_the_slash_event_once() {
        let mut app = input_app(Mode::Playing, true);
        press(&mut app, KeyCode::Slash, Some("/"));
        app.update();

        assert_eq!(app.world().resource::<Game>().mode, Mode::Console);
        assert!(app.world().resource::<Game>().ui_dirty);
        assert_eq!(app.world().resource::<ConsoleState>().input, "/");
    }

    #[test]
    fn console_does_not_open_without_a_world_or_outside_playing() {
        for (mode, has_world) in [
            (Mode::Playing, false),
            (Mode::Dead, true),
            (Mode::Paused, true),
            (Mode::Inventory, true),
        ] {
            let mut app = input_app(mode, has_world);
            press(&mut app, KeyCode::Slash, Some("/"));
            app.update();

            assert_eq!(app.world().resource::<Game>().mode, mode);
            assert!(app.world().resource::<ConsoleState>().input.is_empty());
        }
    }

    #[test]
    fn valid_enter_emits_an_action_and_returns_to_playing() {
        let mut app = input_app(Mode::Console, true);
        app.world_mut().resource_mut::<ConsoleState>().input = "/gamemode creative".into();
        press(&mut app, KeyCode::Enter, Some("\r"));
        app.update();

        assert_eq!(app.world().resource::<Game>().mode, Mode::Playing);
        assert!(app.world().resource::<ConsoleState>().input.is_empty());
        let messages = app.world().resource::<Messages<PlayerActionMessage>>();
        let mut cursor = messages.get_cursor();
        let received: Vec<_> = cursor.read(messages).collect();
        assert_eq!(received.len(), 1);
        assert!(matches!(
            &received[0].0,
            PlayerAction::SetMode(GameMode::Creative)
        ));
    }

    #[test]
    fn malformed_command_stays_open_and_escape_clears_the_input() {
        let mut app = input_app(Mode::Console, true);
        app.world_mut().resource_mut::<ConsoleState>().input = "/gamerule keepInventory yes".into();
        press(&mut app, KeyCode::Enter, None);
        app.update();

        assert_eq!(app.world().resource::<Game>().mode, Mode::Console);
        assert!(app.world().resource::<Game>().ui_dirty);
        assert!(!app.world().resource::<ConsoleState>().feedback.is_empty());
        assert!(
            app.world()
                .resource::<Messages<PlayerActionMessage>>()
                .is_empty()
        );

        press(&mut app, KeyCode::Escape, None);
        app.update();
        assert_eq!(app.world().resource::<Game>().mode, Mode::Playing);
        assert!(app.world().resource::<ConsoleState>().input.is_empty());
        assert!(app.world().resource::<ConsoleState>().feedback.is_empty());
    }

    #[test]
    fn unicode_input_is_single_line_limited_to_256_characters_and_backspaces_safely() {
        let mut app = input_app(Mode::Console, true);
        app.world_mut().resource_mut::<ConsoleState>().input = format!("/{}", "я".repeat(254));
        press(&mut app, KeyCode::KeyA, Some("🦀\n\r\t\0\u{2028}\u{2029}z"));
        app.update();
        let console = app.world().resource::<ConsoleState>();
        assert_eq!(console.input.chars().count(), 256);
        assert!(console.input.ends_with('🦀'));
        assert!(!console.input.chars().any(char::is_control));

        press(&mut app, KeyCode::Backspace, None);
        app.update();
        let console = app.world().resource::<ConsoleState>();
        assert_eq!(console.input.chars().count(), 255);
        assert!(console.input.ends_with('я'));
    }

    #[test]
    fn parses_summon_for_every_mob_and_time_of_day() {
        for kind in game_core::mobs::MobKind::ALL {
            let command = format!("/summon {}", kind.spec().id);
            assert!(
                matches!(parse_command(&command), Ok(PlayerAction::Summon(found)) if found == kind)
            );
        }
        assert!(matches!(
            parse_command("/time day"),
            Ok(PlayerAction::SetDay(true))
        ));
        assert!(matches!(
            parse_command("/time night"),
            Ok(PlayerAction::SetDay(false))
        ));
        for invalid in [
            "/summon",
            "/summon dragon",
            "/summon cow extra",
            "/time",
            "/time noon",
        ] {
            assert!(parse_command(invalid).is_err(), "{invalid}");
        }
    }
}
