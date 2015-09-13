use utils::bit_utils::to_u16;
use arch::instrs::*;
use arch::cpu::CPU;

pub static INSTR_TABLE: [Instr; 256] = [
    error_fn, //00
    ora::indirect_x, //01
    error_fn, //02
    error_fn, //03
    error_fn, //04
    ora::zeropage, //05
    error_fn, //06
    error_fn, //07
    error_fn, //08
    ora::immediate, //09
    error_fn, //0a
    error_fn, //0b
    error_fn, //0c
    ora::absolute, //0d
    error_fn, //0e
    error_fn, //0f
    branches::bpl, //10
    ora::indirect_y, //11
    error_fn, //12
    error_fn, //13
    error_fn, //14
    ora::zeropage_x, //15
    error_fn, //16
    error_fn, //17
    error_fn, //18
    ora::absolute_y, //19
    error_fn, //1a
    error_fn, //1b
    error_fn, //1c
    ora::absolute_x, //1d
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
    branches::bmi, //30
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
    eor::indirect_x, //41
    error_fn, //42
    error_fn, //43
    error_fn, //44
    eor::zeropage, //45
    error_fn, //46
    error_fn, //47
    error_fn, //48
    eor::immediate, //49
    error_fn, //4a
    error_fn, //4b
    jmp::absolute, //4c
    eor::absolute, //4d
    error_fn, //4e
    error_fn, //4f
    branches::bvc, //50
    eor::indirect_y, //51
    error_fn, //52
    error_fn, //53
    error_fn, //54
    eor::zeropage_x, //55
    error_fn, //56
    error_fn, //57
    error_fn, //58
    eor::absolute_y, //59
    error_fn, //5a
    error_fn, //5b
    error_fn, //5c
    eor::absolute_x, //5d
    error_fn, //5e
    error_fn, //5f
    error_fn, //60
    adc::indirect_x, //61
    error_fn, //62
    error_fn, //63
    error_fn, //64
    adc::zeropage, //65
    error_fn, //66
    error_fn, //67
    error_fn, //68
    adc::immediate, //69
    error_fn, //6a
    error_fn, //6b
    jmp::indirect_absolute, //6c
    adc::absolute, //6d
    error_fn, //6e
    error_fn, //6f
    branches::bvs, //70
    adc::indirect_y, //71
    error_fn, //72
    error_fn, //73
    error_fn, //74
    adc::zeropage_x, //75
    error_fn, //76
    error_fn, //77
    error_fn, //78
    adc::absolute_y, //79
    error_fn, //7a
    error_fn, //7b
    error_fn, //7c
    adc::absolute_x, //7d
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
    dey::implied, //88
    error_fn, //89
    error_fn, //8a
    error_fn, //8b
    sty::absolute, //8c
    sta::absolute, //8d
    stx::absolute, //8e
    error_fn, //8f
    branches::bcc, //90
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
    branches::bcs, //b0
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
    cpy::immediate, //c0
    cmp::indirect_x, //c1
    error_fn, //c2
    error_fn, //c3
    cpy::zeropage, //c4
    cmp::zeropage, //c5
    dec::zeropage, //c6
    error_fn, //c7
    iny::implied, //c8
    cmp::immediate, //c9
    dex::implied, //ca
    error_fn, //cb
    cpy::absolute, //cc
    cmp::absolute, //cd
    dec::absolute, //ce
    error_fn, //cf
    branches::bne, //d0
    cmp::indirect_y, //d1
    error_fn, //d2
    error_fn, //d3
    error_fn, //d4
    cmp::zeropage_x, //d5
    dec::zeropage_x, //d6
    error_fn, //d7
    error_fn, //d8
    cmp::absolute_y, //d9
    error_fn, //da
    error_fn, //db
    error_fn, //dc
    cmp::absolute_x, //dd
    dec::absolute_x, //de
    error_fn, //df
    cpx::immediate, //e0
    sbc::indirect_x, //e1
    error_fn, //e2
    error_fn, //e3
    cpx::zeropage, //e4
    sbc::zeropage, //e5
    inc::zeropage, //e6
    error_fn, //e7
    inx::implied, //e8
    sbc::immediate, //e9
    error_fn, //ea
    error_fn, //eb
    cpx::absolute, //ec
    sbc::absolute, //ed
    inc::absolute, //ee
    error_fn, //ef
    branches::beq, //f0
    sbc::indirect_y, //f1
    error_fn, //f2
    error_fn, //f3
    error_fn, //f4
    sbc::zeropage_x, //f5
    inc::zeropage_x, //f6
    error_fn, //f7
    error_fn, //f8
    sbc::absolute_y, //f9
    error_fn, //fa
    error_fn, //fb
    error_fn, //fc
    sbc::absolute_x, //fd
    inc::absolute_x, //fe
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
    (to_u16(low, high).wrapping_add(offset as u16), 3)
}

pub fn decode_zeropage_indexed(cpu: &CPU, offset: u8) -> (u8, u8)
{
    let addr = cpu.memory.borrow().fetch(cpu.registers.PC + 1);
    (addr.wrapping_add(offset), 2)
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

    ((to_u16(low, high).wrapping_add(cpu.registers.Y as u16)) & 0xFFFF, 2)
}
