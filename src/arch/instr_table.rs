use arch::bit_utils::to_u16;
use arch::instrs::*;
use arch::cpu::CPU;

pub static INSTR_TABLE: [Instr; 256] = [
    error_fn, //0
    error_fn, //1
    error_fn, //2
    error_fn, //3
    error_fn, //4
    error_fn, //5
    error_fn, //6
    error_fn, //7
    error_fn, //8
    error_fn, //9
    error_fn, //a
    error_fn, //b
    error_fn, //c
    error_fn, //d
    error_fn, //e
    error_fn, //f
    error_fn, //10
    error_fn, //11
    error_fn, //12
    error_fn, //13
    error_fn, //14
    error_fn, //15
    error_fn, //16
    error_fn, //17
    error_fn, //18
    error_fn, //19
    error_fn, //1a
    error_fn, //1b
    error_fn, //1c
    error_fn, //1d
    error_fn, //1e
    error_fn, //1f
    error_fn, //20
    error_fn, //21
    error_fn, //22
    error_fn, //23
    error_fn, //24
    error_fn, //25
    error_fn, //26
    error_fn, //27
    error_fn, //28
    error_fn, //29
    error_fn, //2a
    error_fn, //2b
    error_fn, //2c
    error_fn, //2d
    error_fn, //2e
    error_fn, //2f
    error_fn, //30
    error_fn, //31
    error_fn, //32
    error_fn, //33
    error_fn, //34
    error_fn, //35
    error_fn, //36
    error_fn, //37
    error_fn, //38
    error_fn, //39
    error_fn, //3a
    error_fn, //3b
    error_fn, //3c
    error_fn, //3d
    error_fn, //3e
    error_fn, //3f
    error_fn, //40
    error_fn, //41
    error_fn, //42
    error_fn, //43
    error_fn, //44
    error_fn, //45
    error_fn, //46
    error_fn, //47
    error_fn, //48
    error_fn, //49
    error_fn, //4a
    error_fn, //4b
    error_fn, //4c
    error_fn, //4d
    error_fn, //4e
    error_fn, //4f
    error_fn, //50
    error_fn, //51
    error_fn, //52
    error_fn, //53
    error_fn, //54
    error_fn, //55
    error_fn, //56
    error_fn, //57
    error_fn, //58
    error_fn, //59
    error_fn, //5a
    error_fn, //5b
    error_fn, //5c
    error_fn, //5d
    error_fn, //5e
    error_fn, //5f
    error_fn, //60
    error_fn, //61
    error_fn, //62
    error_fn, //63
    error_fn, //64
    error_fn, //65
    error_fn, //66
    error_fn, //67
    error_fn, //68
    error_fn, //69
    error_fn, //6a
    error_fn, //6b
    error_fn, //6c
    error_fn, //6d
    error_fn, //6e
    error_fn, //6f
    error_fn, //70
    error_fn, //71
    error_fn, //72
    error_fn, //73
    error_fn, //74
    error_fn, //75
    error_fn, //76
    error_fn, //77
    error_fn, //78
    error_fn, //79
    error_fn, //7a
    error_fn, //7b
    error_fn, //7c
    error_fn, //7d
    error_fn, //7e
    error_fn, //7f
    error_fn, //80
    sta::indirect_x, //81
    error_fn, //82
    error_fn, //83
    sty::zeropage, //84
    sta::zeropage, //85
    stx::zeropage, //86
    error_fn, //87
    error_fn, //88
    error_fn, //89
    error_fn, //8a
    error_fn, //8b
    sty::absolute, //8c
    sta::absolute, //8d
    stx::absolute, //8e
    error_fn, //8f
    error_fn, //90
    sta::indirect_y, //91
    error_fn, //92
    error_fn, //93
    sty::zeropage_x, //94
    sta::zeropage_x, //95
    stx::zeropage_y, //96
    error_fn, //97
    error_fn, //98
    sta::absolute_y, //99
    error_fn, //9a
    error_fn, //9b
    error_fn, //9c
    sta::absolute_x, //9d
    error_fn, //9e
    error_fn, //9f
    ldy::immediate, //a0
    lda::indirect_x, //a1
    ldx::immediate, //a2
    error_fn, //a3
    ldy::zeropage, //a4
    lda::zeropage, //a5
    ldx::zeropage, //a6
    error_fn, //a7
    error_fn, //a8
    lda::immediate, //a9
    error_fn, //aa
    error_fn, //ab
    ldy::absolute, //ac
    lda::absolute, //ad
    ldx::absolute, //ae
    error_fn, //af
    error_fn, //b0
    lda::indirect_y, //b1
    error_fn, //b2
    error_fn, //b3
    ldy::zeropage_x, //b4
    lda::zeropage_x, //b5
    ldx::zeropage_y, //b6
    error_fn, //b7
    error_fn, //b8
    lda::absolute_y, //b9
    error_fn, //ba
    error_fn, //bb
    ldy::absolute_x, //bc
    lda::absolute_x, //bd
    ldx::absolute_y, //be
    error_fn, //bf
    error_fn, //c0
    error_fn, //c1
    error_fn, //c2
    error_fn, //c3
    error_fn, //c4
    error_fn, //c5
    error_fn, //c6
    error_fn, //c7
    error_fn, //c8
    error_fn, //c9
    error_fn, //ca
    error_fn, //cb
    error_fn, //cc
    error_fn, //cd
    error_fn, //ce
    error_fn, //cf
    error_fn, //d0
    error_fn, //d1
    error_fn, //d2
    error_fn, //d3
    error_fn, //d4
    error_fn, //d5
    error_fn, //d6
    error_fn, //d7
    error_fn, //d8
    error_fn, //d9
    error_fn, //da
    error_fn, //db
    error_fn, //dc
    error_fn, //dd
    error_fn, //de
    error_fn, //df
    error_fn, //e0
    error_fn, //e1
    error_fn, //e2
    error_fn, //e3
    error_fn, //e4
    error_fn, //e5
    error_fn, //e6
    error_fn, //e7
    error_fn, //e8
    error_fn, //e9
    error_fn, //ea
    error_fn, //eb
    error_fn, //ec
    error_fn, //ed
    error_fn, //ee
    error_fn, //ef
    error_fn, //f0
    error_fn, //f1
    error_fn, //f2
    error_fn, //f3
    error_fn, //f4
    error_fn, //f5
    error_fn, //f6
    error_fn, //f7
    error_fn, //f8
    error_fn, //f9
    error_fn, //fa
    error_fn, //fb
    error_fn, //fc
    error_fn, //fd
    error_fn, //fe
    error_fn  //ff
];

pub type Instr = fn(&mut CPU) -> (u8, u8);

pub fn error_fn(_cpu: &mut CPU) -> (u8, u8)
{
    //panic!("Invalid opcode!");
    (0xFF, 0xFF)
}

// decode functions
pub fn decode_absolute(cpu: &CPU) -> (u16, u8)
{
    let low = cpu.memory.borrow().fetch(cpu.registers.PC + 1);
    let high = cpu.memory.borrow().fetch(cpu.registers.PC + 2);
    (to_u16(low, high), 3)
}

pub fn decode_immediate(cpu: &CPU) -> (u8, u8)
{
    (cpu.memory.borrow().fetch(cpu.registers.PC + 1), 2)
}

pub fn decode_zeropage(cpu: &CPU) -> (u8, u8)
{
    (cpu.memory.borrow().fetch(cpu.registers.PC + 1), 2)
}

pub fn decode_absolute_indexed(cpu: &CPU, offset: u8) -> (u16, u8)
{
    let low = cpu.memory.borrow().fetch(cpu.registers.PC + 1);
    let high = cpu.memory.borrow().fetch(cpu.registers.PC + 2);
    ((to_u16(low, high) + offset as u16) & 0xFFFF, 3)
}

pub fn decode_zeropage_indexed(cpu: &CPU, offset: u8) -> (u8, u8)
{
    let addr = cpu.memory.borrow().fetch(cpu.registers.PC + 1);
    ((addr + offset) & 0xFF, 2)
}

pub fn decode_indexed_indirect(cpu: &CPU) -> (u16, u8)
{
    let op = (cpu.memory.borrow().fetch(cpu.registers.PC + 1) + cpu.registers.X) as u16 & 0xFF;
    let low = cpu.memory.borrow().fetch(op);
    let high = cpu.memory.borrow().fetch((op + 1) & 0xFF);

    (to_u16(low, high), 2)
}

pub fn decode_indirect_indexed(cpu: &CPU) -> (u16, u8)
{
    let op = cpu.memory.borrow().fetch(cpu.registers.PC + 1) as u16;
    let low = cpu.memory.borrow().fetch(op);
    let high = cpu.memory.borrow().fetch((op + 1) & 0xFF);

    ((to_u16(low, high) + cpu.registers.Y as u16) & 0xFFFF, 2)
}
