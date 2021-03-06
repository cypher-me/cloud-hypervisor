//
// Copyright © 2020 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0
//

extern crate iced_x86;

use crate::arch::emulator::{EmulationError, PlatformEmulator, PlatformError};
use crate::arch::x86::emulator::CpuStateManager;
use crate::arch::x86::Exception;
use iced_x86::*;

macro_rules! imm_op {
    (u8, $insn:ident) => {
        $insn.immediate8()
    };

    (u16, $insn:ident) => {
        $insn.immediate16()
    };

    (u32, $insn:ident) => {
        $insn.immediate32()
    };

    (u64, $insn:ident) => {
        $insn.immediate64()
    };

    (u32tou64, $insn:ident) => {
        $insn.immediate32to64()
    };

    (u8tou16, $insn:ident) => {
        $insn.immediate8to16()
    };

    (u8tou32, $insn:ident) => {
        $insn.immediate8to32()
    };

    (u8tou64, $insn:ident) => {
        $insn.immediate8to64()
    };
}

pub mod mov;

// Returns the linear a.k.a. virtual address for a memory operand.
fn memory_operand_address<T: CpuStateManager>(
    insn: &Instruction,
    state: &T,
    write: bool,
) -> Result<u64, PlatformError> {
    let mut address: u64 = 0;

    if insn.memory_base() != iced_x86::Register::None {
        let base: u64 = state.read_reg(insn.memory_base())?;
        address += base;
    }

    if insn.memory_index() != iced_x86::Register::None {
        let mut index: u64 = state.read_reg(insn.memory_index())?;
        index *= insn.memory_index_scale() as u64;

        address += index;
    }

    address += insn.memory_displacement() as u64;

    // Translate to a linear address.
    state.linearize(insn.memory_segment(), address, write)
}

pub trait InstructionHandler<T: CpuStateManager> {
    fn emulate(
        &self,
        insn: &Instruction,
        state: &mut T,
        platform: &mut dyn PlatformEmulator<CpuState = T>,
    ) -> Result<(), EmulationError<Exception>>;
}

macro_rules! insn_format {
    ($insn:ident) => {{
        let mut output = String::new();
        let mut formatter = FastFormatter::new();
        formatter
            .options_mut()
            .set_space_after_operand_separator(true);
        formatter.format(&$insn, &mut output);

        output
    }};
}
