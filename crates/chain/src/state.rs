use std::collections::BTreeMap;

use game::{Action, Event, GameError, GameState, PlayerId};
use thiserror::Error;

use crate::Encode;
use crate::encode::encode_len;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub sender: PlayerId,
    pub nonce: u64,
    pub action: Action,
}

impl Encode for Transaction {
    fn encode(&self, out: &mut Vec<u8>) {
        self.sender.encode(out);
        self.nonce.encode(out);
        self.action.encode(out);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TxError {
    #[error("ожидался номер хода {expected}, получен {got}")]
    BadNonce { expected: u64, got: u64 },
    #[error("счётчик ходов игрока переполнен")]
    NonceOverflow,
    #[error("ход отклонён правилами игры: {0}")]
    Game(#[from] GameError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainState {
    game: GameState,
    nonces: BTreeMap<PlayerId, u64>,
}

impl ChainState {
    /// Применяет транзакцию по правилам цепочки.
    ///
    /// # Errors
    ///
    /// /* три отказа из правил цепочки */
    pub fn apply(&mut self, tx: &Transaction) -> Result<Vec<Event>, TxError> {
        let expected = self.next_nonce(tx.sender);
        if tx.nonce != expected {
            return Err(TxError::BadNonce {
                expected,
                got: tx.nonce,
            });
        }
        let next = expected.checked_add(1).ok_or(TxError::NonceOverflow)?;

        // Используется clone, так как apply забирает action во владение, а транзакция должна остаться целой для корректного хеширвоания
        let events = game::apply(&mut self.game, tx.sender, tx.action.clone())?;

        self.nonces.insert(tx.sender, next);
        Ok(events)
    }

    #[must_use]
    pub fn next_nonce(&self, player: PlayerId) -> u64 {
        self.nonces.get(&player).copied().unwrap_or(0)
    }

    #[must_use]
    pub fn game(&self) -> &GameState {
        &self.game
    }

    #[must_use]
    pub fn genesis() -> ChainState {
        ChainState {
            game: GameState::genesis(),
            nonces: BTreeMap::new(),
        }
    }
}

impl Encode for ChainState {
    fn encode(&self, out: &mut Vec<u8>) {
        self.game().encode(out);
        encode_len(self.nonces.len(), out);
        for (id, player) in &self.nonces {
            id.encode(out);
            player.encode(out);
        }
    }
}
