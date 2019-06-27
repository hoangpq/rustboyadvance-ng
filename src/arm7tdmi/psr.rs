/// The program status register
use std::fmt;

use crate::bit::BitIndex;
use crate::num_traits::FromPrimitive;

use colored::*;

use super::arm::ArmCond;

#[derive(Debug, PartialEq, Primitive)]
#[repr(u8)]
pub enum CpuState {
    ARM = 0,
    THUMB = 1,
}

impl fmt::Display for CpuState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CpuState::*;
        match self {
            ARM => write!(f, "ARM"),
            THUMB => write!(f, "THUMB"),
        }
    }
}

impl From<CpuState> for bool {
    fn from(state: CpuState) -> bool {
        match state {
            CpuState::ARM => false,
            CpuState::THUMB => true,
        }
    }
}

impl From<bool> for CpuState {
    fn from(flag: bool) -> CpuState {
        if flag {
            CpuState::THUMB
        } else {
            CpuState::ARM
        }
    }
}

#[derive(Debug, Primitive)]
#[repr(u8)]
pub enum CpuMode {
    User = 0b10000,
    Fiq = 0b10001,
    Irq = 0b10010,
    Supervisor = 0b10011,
    Abort = 0b10111,
    Undefined = 0b11011,
    System = 0b11111,
}

impl fmt::Display for CpuMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CpuMode::*;
        match self {
            User => write!(f, "USR"),
            Fiq => write!(f, "FIQ"),
            Irq => write!(f, "IRQ"),
            Supervisor => write!(f, "SVC"),
            Abort => write!(f, "ABT"),
            Undefined => write!(f, "UND"),
            System => write!(f, "SYS"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RegPSR {
    raw: u32,
}

const RESERVED_BIT_MASK: u32 = 0x0fffff00;
fn clear_reserved(n: u32) -> u32 {
    n & !RESERVED_BIT_MASK
}

impl RegPSR {
    pub fn new() -> RegPSR {
        let mut psr = RegPSR { raw: 0 };

        psr.set_irq_disabled(true);
        psr.set_fiq_disabled(true);
        psr.set_mode(CpuMode::Supervisor);
        psr.set_state(CpuState::ARM);
        println!("RAW: 0x{:08x}", psr.raw);

        psr
    }

    pub fn get(&self) -> u32 {
        self.raw
    }

    pub fn set(&mut self, psr: u32) {
        self.raw = clear_reserved(psr);
    }

    pub fn state(&self) -> CpuState {
        self.raw.bit(5).into()
    }

    pub fn set_state(&mut self, state: CpuState) {
        self.raw.set_bit(5, state.into());
    }

    pub fn mode(&self) -> CpuMode {
        CpuMode::from_u32(self.raw & 0xb11111).unwrap()
    }

    pub fn set_mode(&mut self, mode: CpuMode) {
        self.raw |= mode as u32;
    }

    pub fn irq_disabled(&self) -> bool {
        self.raw.bit(7)
    }

    pub fn set_irq_disabled(&mut self, disabled: bool) {
        self.raw.set_bit(7, disabled);
    }

    pub fn fiq_disabled(&self) -> bool {
        self.raw.bit(6)
    }

    pub fn set_fiq_disabled(&mut self, disabled: bool) {
        self.raw.set_bit(6, disabled);
    }

    #[allow(non_snake_case)]
    pub fn N(&self) -> bool {
        self.raw.bit(31)
    }

    #[allow(non_snake_case)]
    pub fn set_N(&mut self, flag: bool) {
        self.raw.set_bit(31, flag);
    }

    #[allow(non_snake_case)]
    pub fn Z(&self) -> bool {
        self.raw.bit(30)
    }

    #[allow(non_snake_case)]
    pub fn set_Z(&mut self, flag: bool) {
        self.raw.set_bit(30, flag);
    }

    #[allow(non_snake_case)]
    pub fn C(&self) -> bool {
        self.raw.bit(29)
    }

    #[allow(non_snake_case)]
    pub fn set_C(&mut self, flag: bool) {
        self.raw.set_bit(29, flag);
    }

    #[allow(non_snake_case)]
    pub fn V(&self) -> bool {
        self.raw.bit(28)
    }

    #[allow(non_snake_case)]
    pub fn set_V(&mut self, flag: bool) {
        self.raw.set_bit(28, flag);
    }
}

impl fmt::Display for RegPSR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let disabled_string = |disabled: bool| -> ColoredString {
            if disabled {
                "disabled".bright_red()
            } else {
                "enabled".bright_green()
            }
        };
        write!(
            f,
            "{{ mode: {mode}, state: {state}, irq: {irq}, fiq: {fiq}, condition_flags: (N={N} Z={Z} C={C} V={V}) }}",
            mode = self.mode(),
            state = self.state(),
            irq = disabled_string(self.irq_disabled()),
            fiq = disabled_string(self.irq_disabled()),
            N = self.N() as u8,
            Z = self.Z() as u8,
            C = self.C() as u8,
            V = self.V() as u8,
            )
    }
}