use std::{borrow::Cow, collections::HashSet};

use super::{Instr, Opcode};
use itertools::Itertools;
use wasm_encoder::{
    BlockType, CodeSection, ConstExpr, EntityType, ExportKind, ExportSection, Function,
    FunctionSection, GlobalSection, GlobalType, ImportSection, Instruction as Ins, Module,
    TypeSection, ValType,
};

pub fn emit(callbacks: impl IntoIterator<Item = i64>, prog: &[Instr], ip_reg: usize) -> Vec<u8> {
    let callbacks = HashSet::<i64>::from_iter(callbacks.into_iter());
    let mut module = Module::new();

    let mut types = TypeSection::new();
    types.function(
        [
            ValType::I64,
            ValType::I64,
            ValType::I64,
            ValType::I64,
            ValType::I64,
            ValType::I64,
            ValType::I64,
        ],
        [ValType::I32],
    );
    types.function([], []);
    module.section(&types);

    let mut imports = ImportSection::new();
    imports.import("env", "callback", EntityType::Function(0));
    module.section(&imports);

    let mut functions = FunctionSection::new();
    functions.function(1);
    module.section(&functions);

    let mut globals = GlobalSection::new();
    for _ in 0..7 {
        globals.global(
            GlobalType {
                val_type: ValType::I64,
                mutable: true,
            },
            &ConstExpr::i64_const(0),
        );
    }
    module.section(&globals);

    let mut exports = ExportSection::new();
    exports.export("run", ExportKind::Func, 1);
    for i in 0..6 {
        exports.export(&format!("r{i}"), ExportKind::Global, i);
    }
    exports.export("pc", ExportKind::Global, 6);
    module.section(&exports);

    let mut codes = CodeSection::new();
    // locals: 6x register, pc
    let locals = vec![(7, ValType::I64)];
    let mut f = Function::new(locals);
    for i in 0..7 {
        f.instruction(&Ins::GlobalGet(i as u32));
        f.instruction(&Ins::LocalSet(i as u32));
    }
    f.instruction(&Ins::Loop(BlockType::Empty));
    let proglen = prog.len() as u32;
    for _ in 0..=proglen + 1 {
        f.instruction(&Ins::Block(BlockType::Empty));
    }
    f.instruction(&Ins::LocalGet(6));
    f.instruction(&Ins::I32WrapI64);
    f.instruction(&Ins::BrTable(
        Cow::Owned((0..proglen).collect_vec()),
        proglen,
    ));
    f.instruction(&Ins::End);
    for idx in 0..proglen {
        emit_block(&callbacks, &mut f, prog, ip_reg, idx);
        f.instruction(&Ins::Br(proglen - idx + 1));
        f.instruction(&Ins::End);
    }
    f.instruction(&Ins::End);
    f.instruction(&Ins::End);
    spill_to_globals(&mut f);
    f.instruction(&Ins::End);
    codes.function(&f);
    module.section(&codes);

    let bytes = module.finish();
    std::fs::write("/tmp/p21.wasm", &bytes).unwrap();
    bytes
}

fn emit_block(callbacks: &HashSet<i64>, f: &mut Function, prog: &[Instr], ip_reg: usize, idx: u32) {
    for (i, instr) in prog[idx as usize..].iter().enumerate() {
        if callbacks.contains(&(idx as i64 + i as i64)) {
            for i in 0..7 {
                f.instruction(&Ins::LocalGet(i as u32));
            }
            f.instruction(&Ins::Call(0));
            f.instruction(&Ins::BrIf(prog.len() as u32 - idx + 2));
        }
        f.instruction(&Ins::LocalGet(6));
        f.instruction(&Ins::LocalSet(ip_reg as u32));
        match instr.cmd {
            Opcode::Addr => {
                f.instruction(&Ins::LocalGet(instr.a as u32));
                f.instruction(&Ins::LocalGet(instr.b as u32));
                f.instruction(&Ins::I64Add);
            }
            Opcode::Addi => {
                f.instruction(&Ins::LocalGet(instr.a as u32));
                f.instruction(&Ins::I64Const(instr.b as i64));
                f.instruction(&Ins::I64Add);
            }
            Opcode::Mulr => {
                f.instruction(&Ins::LocalGet(instr.a as u32));
                f.instruction(&Ins::LocalGet(instr.b as u32));
                f.instruction(&Ins::I64Mul);
            }
            Opcode::Muli => {
                f.instruction(&Ins::LocalGet(instr.a as u32));
                f.instruction(&Ins::I64Const(instr.b as i64));
                f.instruction(&Ins::I64Mul);
            }
            Opcode::Banr => {
                f.instruction(&Ins::LocalGet(instr.a as u32));
                f.instruction(&Ins::LocalGet(instr.b as u32));
                f.instruction(&Ins::I64And);
            }
            Opcode::Bani => {
                f.instruction(&Ins::LocalGet(instr.a as u32));
                f.instruction(&Ins::I64Const(instr.b as i64));
                f.instruction(&Ins::I64And);
            }
            Opcode::Borr => {
                f.instruction(&Ins::LocalGet(instr.a as u32));
                f.instruction(&Ins::LocalGet(instr.b as u32));
                f.instruction(&Ins::I64Or);
            }
            Opcode::Bori => {
                f.instruction(&Ins::LocalGet(instr.a as u32));
                f.instruction(&Ins::I64Const(instr.b as i64));
                f.instruction(&Ins::I64Or);
            }
            Opcode::Setr => {
                f.instruction(&Ins::LocalGet(instr.a as u32));
            }
            Opcode::Seti => {
                f.instruction(&Ins::I64Const(instr.a as i64));
            }
            Opcode::Gtir => {
                f.instruction(&Ins::I64Const(instr.a as i64));
                f.instruction(&Ins::LocalGet(instr.b as u32));
                f.instruction(&Ins::I64GtS);
                f.instruction(&Ins::I64ExtendI32U);
            }
            Opcode::Gtri => {
                f.instruction(&Ins::LocalGet(instr.a as u32));
                f.instruction(&Ins::I64Const(instr.b as i64));
                f.instruction(&Ins::I64GtS);
                f.instruction(&Ins::I64ExtendI32U);
            }
            Opcode::Gtrr => {
                f.instruction(&Ins::LocalGet(instr.a as u32));
                f.instruction(&Ins::LocalGet(instr.b as u32));
                f.instruction(&Ins::I64GtS);
                f.instruction(&Ins::I64ExtendI32U);
            }
            Opcode::Eqir => {
                f.instruction(&Ins::I64Const(instr.a as i64));
                f.instruction(&Ins::LocalGet(instr.b as u32));
                f.instruction(&Ins::I64Eq);
                f.instruction(&Ins::I64ExtendI32U);
            }
            Opcode::Eqri => {
                f.instruction(&Ins::LocalGet(instr.a as u32));
                f.instruction(&Ins::I64Const(instr.b as i64));
                f.instruction(&Ins::I64Eq);
                f.instruction(&Ins::I64ExtendI32U);
            }
            Opcode::Eqrr => {
                f.instruction(&Ins::LocalGet(instr.a as u32));
                f.instruction(&Ins::LocalGet(instr.b as u32));
                f.instruction(&Ins::I64Eq);
                f.instruction(&Ins::I64ExtendI32U);
            }
        }
        f.instruction(&Ins::LocalSet(instr.c as u32));
        f.instruction(&Ins::LocalGet(ip_reg as u32));
        f.instruction(&Ins::I64Const(1));
        f.instruction(&Ins::I64Add);
        f.instruction(&Ins::LocalSet(6));
        if instr.c == ip_reg {
            break;
        }
    }
}

fn spill_to_globals(f: &mut Function) {
    for i in 0..7 {
        f.instruction(&Ins::LocalGet(i as u32));
        f.instruction(&Ins::GlobalSet(i as u32));
    }
}
