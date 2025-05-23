use crate::cpu::instructions::macros::implement_store_instruction;
use crate::{
    cpu::state::{InstructionExecutor, InstructionState},
    memory::{LoadOps, MemAccessSize, MemoryProcessor, StoreOps},
    riscv::Instruction,
};
use nexus_common::cpu::{Processor, Registers};

pub struct ShInstruction {
    rs1: u32,
    rs2: u32,
    imm: u32,
}

implement_store_instruction!(ShInstruction, MemAccessSize::HalfWord);

#[cfg(test)]
mod tests {
    use nexus_common::error::MemoryError;

    use super::*;
    use crate::cpu::state::Cpu;
    use crate::memory::{LoadOp, VariableMemory, RW};
    use crate::riscv::{BuiltinOpcode, Instruction, Opcode, Register};

    fn setup_memory() -> VariableMemory<RW> {
        VariableMemory::<RW>::default()
    }

    #[test]
    fn test_sh_positive_value() {
        let mut cpu = Cpu::default();
        let mut memory = setup_memory();

        cpu.registers.write(Register::X1, 0x1000);
        cpu.registers.write(Register::X2, 0x7FFF);

        let bare_instruction = Instruction::new_ir(Opcode::from(BuiltinOpcode::SH), 1, 2, 0);
        let instruction = ShInstruction::decode(&bare_instruction, &cpu.registers);

        instruction.memory_write(&mut memory).unwrap();
        let res = instruction.write_back(&mut cpu);

        assert_eq!(res, None);
        assert_eq!(
            memory.read(0x1000, MemAccessSize::HalfWord).unwrap(),
            LoadOp::Op(MemAccessSize::HalfWord, 0x1000, 0x7FFF),
        );
    }

    #[test]
    fn test_sh_negative_value() {
        let mut cpu = Cpu::default();
        let mut memory = setup_memory();

        cpu.registers.write(Register::X1, 0x1000);
        cpu.registers.write(Register::X2, 0xFFFF8000); // -32768 in two's complement

        let bare_instruction = Instruction::new_ir(Opcode::from(BuiltinOpcode::SH), 1, 2, 2);
        let instruction = ShInstruction::decode(&bare_instruction, &cpu.registers);

        instruction.memory_write(&mut memory).unwrap();
        let res = instruction.write_back(&mut cpu);

        assert_eq!(res, None);
        assert_eq!(
            memory.read(0x1002, MemAccessSize::HalfWord).unwrap(),
            LoadOp::Op(MemAccessSize::HalfWord, 0x1002, 0x8000),
        );
    }

    #[test]
    fn test_sh_max_value() {
        let mut cpu = Cpu::default();
        let mut memory = setup_memory();

        cpu.registers.write(Register::X1, 0x1000);
        cpu.registers.write(Register::X2, 0xFFFF);

        let bare_instruction = Instruction::new_ir(Opcode::from(BuiltinOpcode::SH), 1, 2, 4);
        let instruction = ShInstruction::decode(&bare_instruction, &cpu.registers);

        instruction.memory_write(&mut memory).unwrap();
        let res = instruction.write_back(&mut cpu);

        assert_eq!(res, None);
        assert_eq!(
            memory.read(0x1004, MemAccessSize::HalfWord).unwrap(),
            LoadOp::Op(MemAccessSize::HalfWord, 0x1004, 0xFFFF),
        );
    }

    #[test]
    fn test_sh_unaligned_address() {
        let mut cpu = Cpu::default();
        let mut memory = setup_memory();

        cpu.registers.write(Register::X1, 0x1001); // Unaligned address
        cpu.registers.write(Register::X2, 0xABCD);

        let bare_instruction = Instruction::new_ir(Opcode::from(BuiltinOpcode::SH), 1, 2, 0);
        let instruction = ShInstruction::decode(&bare_instruction, &cpu.registers);

        let result = instruction.memory_write(&mut memory);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MemoryError::UnalignedMemoryWrite(0x1001)
        ));
    }

    #[test]
    fn test_sh_overflow() {
        let mut cpu = Cpu::default();
        let mut memory = setup_memory();

        cpu.registers.write(Register::X1, u32::MAX);
        cpu.registers.write(Register::X2, 0xABCD);

        let bare_instruction = Instruction::new_ir(Opcode::from(BuiltinOpcode::SH), 1, 2, 1);
        let instruction = ShInstruction::decode(&bare_instruction, &cpu.registers);

        let result = instruction.memory_write(&mut memory);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MemoryError::AddressCalculationOverflow
        ));
    }

    // TODO: depending on the memory model, we need to test out of bound memory access
}
