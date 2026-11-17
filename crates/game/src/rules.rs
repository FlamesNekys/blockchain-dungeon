use crate::{Action, Direction, Event, GameError, GameState, PlayerId, PlayerState};

pub const START_HEALTH: u32 = 100;
pub const BASE_DAMAGE: u32 = 10;
pub const SWORD_DAMAGE: u32 = 25;
pub const POTION_HEAL: u32 = 30;
pub const SWORD: &str = "sword";
pub const POTION: &str = "potion";

pub fn apply(
    state: &mut GameState,
    who: PlayerId,
    action: Action,
) -> Result<Vec<Event>, GameError> {
    let Some(player) = state.player(who) else {
        return Err(GameError::UnknownPlayer(who));
    };
    if !player.is_alive() {
        return Err(GameError::PlayerDead(who));
    }
    if state.turn() == u64::MAX {
        return Err(GameError::TurnOverflow);
    }
    let events = match action {
        Action::Move(direction) => apply_move(state, who, direction)?,
        Action::Wait => vec![Event::Waited { player: who }],
        Action::PickUp => apply_pick_up(state, who)?,
        Action::UseItem { item } => apply_use(state, who, item)?,
        Action::Give { to, item } => apply_give(state, who, to, item)?,
        Action::Attack { target } => apply_attack(state, who, target)?,
    };
    state.turn_count = state.turn() + 1;
    Ok(events)
}

fn living_health(state: &GameState, id: PlayerId) -> Result<u32, GameError> {
    let player = state.player(id).ok_or(GameError::UnknownPlayer(id))?;
    player.health().ok_or(GameError::PlayerDead(id))
}

fn apply_move(
    state: &mut GameState,
    who: PlayerId,
    direction: Direction,
) -> Result<Vec<Event>, GameError> {
    let from = state
        .player(who)
        .ok_or(GameError::UnknownPlayer(who))?
        .position;
    let to = from.neighbor(direction).ok_or(GameError::OutOfBounds)?;
    state.check_can_stand(to)?;
    let actor = state
        .players
        .get_mut(&who)
        .ok_or(GameError::UnknownPlayer(who))?;
    actor.position = to;
    Ok(vec![Event::Moved { player: who, to }])
}

fn apply_pick_up(state: &mut GameState, who: PlayerId) -> Result<Vec<Event>, GameError> {
    let actor = state
        .players
        .get_mut(&who)
        .ok_or(GameError::UnknownPlayer(who))?;
    let item = state
        .world
        .take_item(actor.position)
        .ok_or(GameError::NothingToPickUp)?;
    let event = Event::PickedUp {
        player: who,
        item: item.clone(),
    };
    actor.inventory.add(item);
    Ok(vec![event])
}

fn apply_use(state: &mut GameState, who: PlayerId, item: String) -> Result<Vec<Event>, GameError> {
    let health = living_health(state, who)?;
    let actor = state
        .players
        .get(&who)
        .ok_or(GameError::UnknownPlayer(who))?;
    if !actor.inventory.contains(item.as_str()) {
        return Err(GameError::NoSuchItem(item));
    }

    let actor = state
        .players
        .get_mut(&who)
        .ok_or(GameError::UnknownPlayer(who))?;

    if item == POTION {
        let healed = health.saturating_add(POTION_HEAL).min(START_HEALTH);
        actor.inventory.remove(item.as_str());
        actor.state = PlayerState::Alive { health: healed };
        return Ok(vec![Event::Healed {
            player: who,
            health: healed,
        }]);
    }
    actor.inventory.equip(item.as_str());
    Ok(vec![Event::Equipped { player: who, item }])
}

fn check_target(state: &GameState, who: PlayerId, target: PlayerId) -> Result<u32, GameError> {
    if who == target {
        return Err(GameError::SelfTarget);
    }
    let health = living_health(state, target)?;
    let from = state
        .player(who)
        .ok_or(GameError::UnknownPlayer(who))?
        .position;
    let at = state
        .player(target)
        .ok_or(GameError::UnknownPlayer(who))?
        .position;
    if from.distance(at) > 1 {
        return Err(GameError::TooFar);
    }
    Ok(health)
}

fn apply_give(
    state: &mut GameState,
    who: PlayerId,
    to: PlayerId,
    item: String,
) -> Result<Vec<Event>, GameError> {
    check_target(state, who, to)?;
    let actor = state
        .players
        .get(&who)
        .ok_or(GameError::UnknownPlayer(who))?;
    if !actor.inventory.contains(item.as_str()) {
        return Err(GameError::NoSuchItem(item));
    }

    let giver = state
        .players
        .get_mut(&who)
        .ok_or(GameError::UnknownPlayer(who))?;
    let Some(gift) = giver.inventory.remove(item.as_str()) else {
        return Err(GameError::NoSuchItem(item));
    };
    let receiver = state
        .players
        .get_mut(&to)
        .ok_or(GameError::UnknownPlayer(to))?;
    receiver.inventory.add(gift);
    Ok(vec![Event::Gave {
        from: who,
        to,
        item,
    }])
}

fn apply_attack(
    state: &mut GameState,
    who: PlayerId,
    target: PlayerId,
) -> Result<Vec<Event>, GameError> {
    let health = check_target(state, who, target)?;
    let weapon = state
        .players()
        .get(&who)
        .ok_or(GameError::UnknownPlayer(who))?
        .inventory
        .equipped();
    let amount = if weapon == Some(SWORD) {
        SWORD_DAMAGE
    } else {
        BASE_DAMAGE
    };
    let left = health.saturating_sub(amount);
    let died_at_turn = state.turn_count;

    let victim = state
        .players
        .get_mut(&target)
        .ok_or(GameError::UnknownPlayer(target))?;
    let mut events = vec![Event::Damaged {
        player: target,
        by: who,
        amount,
        health: left,
    }];
    if left == 0 {
        victim.state = PlayerState::Dead { died_at_turn };
        events.push(Event::Died { player: target });
    } else {
        victim.state = PlayerState::Alive { health: left };
    }
    Ok(events)
}
