use std::error::Error;

use cli::{Command, CommandError, parse_command};
use game::{Action, POTION, PlayerId, SWORD};

fn expect_action(line: &str, who: u32, action: Action) {
    let expected = Command {
        who: PlayerId(who),
        action,
    };
    assert_eq!(parse_command(line), Ok(expected), "строка: {line:?}");
}

#[test]
fn every_kind_of_command_is_understood() {
    expect_action("1 move north", 1, Action::Move(game::Direction::North));
    expect_action("1 move south", 1, Action::Move(game::Direction::South));
    expect_action("1 move east", 1, Action::Move(game::Direction::East));
    expect_action("1 move west", 1, Action::Move(game::Direction::West));
    expect_action(
        "1 attack 2",
        1,
        Action::Attack {
            target: PlayerId(2),
        },
    );
    expect_action("1 pick", 1, Action::PickUp);
    expect_action(
        "1 use potion",
        1,
        Action::UseItem {
            item: String::from(POTION),
        },
    );
    expect_action(
        "1 give 2 sword",
        1,
        Action::Give {
            to: PlayerId(2),
            item: String::from(SWORD),
        },
    );
    expect_action("7 wait", 7, Action::Wait);
}

#[test]
fn extra_spaces_do_not_matter() {
    expect_action(
        "    1    move   east    ",
        1,
        Action::Move(game::Direction::East),
    );
}

#[test]
fn empty_line_is_reported() {
    assert_eq!(parse_command(""), Err(cli::CommandError::Empty));
    assert_eq!(parse_command("    "), Err(cli::CommandError::Empty));
}

#[test]
fn bad_numbers_are_reported() {
    assert!(matches!(
        parse_command("anna wait"),
        Err(CommandError::BadNumber(_))
    ));
    assert!(matches!(
        parse_command("-1 wait"),
        Err(CommandError::BadNumber(_))
    ));
    assert!(matches!(
        parse_command("1 attack boris"),
        Err(CommandError::BadNumber(_))
    ));
    assert!(matches!(
        parse_command("1 give boris sword"),
        Err(CommandError::BadNumber(_))
    ));
}

#[test]
fn missing_parts_are_reported() {
    assert_eq!(parse_command("1"), Err(CommandError::MissingVerb));
    assert_eq!(parse_command("1 move"), Err(CommandError::MissingArgument));
    assert_eq!(
        parse_command("1 attack"),
        Err(CommandError::MissingArgument)
    );
    assert_eq!(parse_command("1 use"), Err(CommandError::MissingArgument));
    assert_eq!(parse_command("1 give"), Err(CommandError::MissingArgument));
    assert_eq!(
        parse_command("1 give 2"),
        Err(CommandError::MissingArgument)
    );
}

#[test]
fn unknown_words_are_reported() {
    assert_eq!(
        parse_command("1 fly"),
        Err(CommandError::UnknownVerb(String::from("fly")))
    );
    assert_eq!(
        parse_command("1 move up"),
        Err(CommandError::UnknownDirection(String::from("up")))
    );
}

#[test]
fn extra_words_are_reported() {
    assert_eq!(
        parse_command("1 wait now"),
        Err(CommandError::ExtraArgument(String::from("now")))
    );
    assert_eq!(
        parse_command("1 move north fast"),
        Err(CommandError::ExtraArgument(String::from("fast")))
    );
}

#[test]
fn errors_explain_themselves_and_keep_their_cause() {
    let error = parse_command("x wait").expect_err("x is not a number");
    assert!(error.to_string().starts_with("неправильный номер игрока"));
    assert!(error.source().is_some());
    assert!(CommandError::Empty.source().is_none());
    assert_eq!(
        CommandError::UnknownVerb(String::from("fly")).to_string(),
        "неизвестное действие: fly"
    );
}
