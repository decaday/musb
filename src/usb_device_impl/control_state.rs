use core::sync::atomic::{AtomicU32, AtomicU8, Ordering};
use crate::warn;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum ControlStateEnum {
    Idle,
    Setup,
    DataIn,
    DataOut,
    Accepted,
    NodataPhase,
    // Error,
}

impl From<u8> for ControlStateEnum {
    fn from(value: u8) -> Self {
        match value {
            0 => ControlStateEnum::Idle,
            1 => ControlStateEnum::Setup,
            2 => ControlStateEnum::DataIn,
            3 => ControlStateEnum::DataOut,
            4 => ControlStateEnum::Accepted,
            5 => ControlStateEnum::NodataPhase,
            _ => unreachable!(),
        }
    }
}

pub(super) struct ControlState {
    state: AtomicU8,
    tx_len: AtomicU32,
}

impl ControlState {
    pub(super) const fn new() -> Self {
        Self {
            state: AtomicU8::new(ControlStateEnum::Idle as u8),
            tx_len: AtomicU32::new(0),
        }
    }

    pub(super) fn set_state(&self, state: ControlStateEnum) {
        self.state.store(state as u8, Ordering::SeqCst);
    }

    pub(super) fn get_state(&self) -> ControlStateEnum {
        ControlStateEnum::from(self.state.load(Ordering::SeqCst))
    }

    pub(super) fn reset_tx_len(&self) {
        self.tx_len.store(0, Ordering::SeqCst);
    }

    pub(super) fn set_tx_len(&self, tx_len: u32) {
        self.tx_len.store(tx_len, Ordering::SeqCst);
    }

    pub(super) fn decrease_tx_len(&self, len: u32) {
        let tx_len = self.tx_len.load(Ordering::SeqCst);
        if len > tx_len {
            warn!("decrease_tx_len: len {} > tx_len {}", len, tx_len);
            self.tx_len.store(0, Ordering::SeqCst);
        }
        else {
            self.tx_len.store(tx_len - len, Ordering::SeqCst);
        }
    }

    pub(super) fn get_tx_len(&self) -> u32 {
        self.tx_len.load(Ordering::SeqCst)
    }
}
