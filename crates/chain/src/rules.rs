use game::{Action, GameState};

use crate::Transaction;

pub trait Rule {
    fn name(&self) -> &'static str;

    /// Проверяет состояние игры и транзакцию на соответствие правилу
    ///
    /// # Errors
    ///
    /// /* текст причины при нарушении */
    fn check(
        &self,
        state: &GameState,
        earlier: &[Transaction],
        tx: &Transaction,
    ) -> Result<(), String>;
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct MaxActionsPerPlayer {
    pub limit: usize,
}

impl Rule for MaxActionsPerPlayer {
    fn name(&self) -> &'static str {
        "max-actions-per-player"
    }

    fn check(
        &self,
        _: &GameState,
        earlier: &[Transaction],
        tx: &Transaction,
    ) -> Result<(), String> {
        let done = earlier
            .iter()
            .filter(|other| other.sender == tx.sender)
            .count();
        if done < self.limit {
            Ok(())
        } else {
            Err(format!(
                "Игрок превысил допустимое количество ходов ({} из {})",
                done, self.limit
            ))
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct NoWaiting;

impl Rule for NoWaiting {
    fn name(&self) -> &'static str {
        "no-waiting"
    }

    fn check(&self, _: &GameState, _: &[Transaction], tx: &Transaction) -> Result<(), String> {
        if tx.action == Action::Wait {
            Err("Ожидание запрещено".to_string())
        } else {
            Ok(())
        }
    }
}
