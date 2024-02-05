//! This ROM was painstakingly optically copied from the chip, bit by bit.

use arbitrary_int::{u11,u4};

fn get_word(rom: &[u64;55], addr: u16) -> u11 {

    let addr_high = addr >> 6;
    let x_addr = addr & 0b111111;
    
    let (y1, y2) = match addr_high {
        0 => (4, 5),
        1 => (3, 6),
        2 => (2, 7),
        3 => (1, 8),
        _ => (0, 9),
    };

    let bita = (!(rom[y1] >> x_addr) & 0b1) as u16;
    let bit9 = (!(rom[y2] >> x_addr) & 0b1) as u16;
    let bit8 = (!(rom[10 + y1] >> x_addr) & 0b1) as u16;
    let bit7 = (!(rom[10 + y2] >> x_addr) & 0b1) as u16;
    let bit6 = (!(rom[20 + y1] >> x_addr) & 0b1) as u16;
    let bit5 = (!(rom[20 + y2] >> x_addr) & 0b1) as u16;
    let bit4 = (!(rom[30 + y1] >> x_addr) & 0b1) as u16;
    let bit3 = (!(rom[30 + y2] >> x_addr) & 0b1) as u16;
    let bit2 = (!(rom[40 + y1] >> x_addr) & 0b1) as u16;
    let bit1 = (!(rom[40 + y2] >> x_addr) & 0b1) as u16;
    let bit0 = (!(rom[50 + y1] >> x_addr) & 0b1) as u16;
    u11::new((bita << 10) + (bit9 << 9) + (bit8 << 8) + (bit7 << 7) + (bit6 << 6) + (bit5 << 5) + (bit4 << 4) + (bit3 << 3) + (bit2 << 2) + (bit1 << 1) + bit0)
}

pub fn decode() -> [u11; 320] {
  let rom: [u64;55] = [
    0b1110010100100101000101011100100100000010001000110010101010001001,
    0b0000101010001010010010100010101001001010000100100100100010100010,
    0b1010100001001001010000010010100000100010101010100010101000010000,
    0b1010000000000000010001001001010100010010100100101000010000001010,
    0b1000001000100001001000000100100100101000000001001010100101010000,
    0b1111001001111100000111100011011010100000000101000010100110010011,
    0b1100001111111111011110101110111111110010001111110111111010111101,
    0b1010000101011111101100111011011010100011101011000001110000011101,
    0b0000111110001010000110000000111000001000000011111111111010111010,
    0b1100011100100100000111011100000001111111011110110010000100011000,

    0b1110010110010111110001111001011110000000100001101001010001100100,
    0b1001101011001111111011110111101101101111101100100100100111100011,
    0b1110111001101001010010010110100100110110111110101010101000010000,
    0b1010001111111110010001011001010100011001110100101000010100001010,
    0b1000011100100001101000001100100101101110100001101111100101110000,
    0b1011011011101111001111111111111101111011111011011010100101011110,
    0b0011011111111111111111011011010100111110001110101011110111111010,
    0b0101000010000010000011100100000101111100101000110100001000000010,
    0b0111110101110001000101001100010010000100111001000001001101000100,
    0b1110010001111000001101111110101100110010111001111110101011111011,

    0b1110011110110101011111111101100101000110111111111010101111111101,
    0b0110110001000001100100001100010100010101111010010011010100000100,
    0b0110101011011101101001011001011111110111101011110111111001111111,
    0b1111110000010010110001101101111111001010100001010111111101000101,
    0b0111001111111011000001010100100111101111111111101010100111011101,
    0b1000100000011100010010000001001010111100110111101111111101110101,
    0b1010000101101111100110111111011101111101111111101000011111101010,
    0b1100011000100010110010001000001111101110111111101010101110010111,
    0b1111100001100101000000001000101100011110110001111010000110110011,
    0b1111110010011011111001110001010111100011001110011111111101101101,

    0b1101101100001100001011001100100111100000100010110100001000101001,
    0b0001001010110010101010001100000000000100111111010010010110100011,
    0b1000010000100110011010010101011011001001000001000001010110001111,
    0b1100100011101010011001010110101011100110001011010100011011101111,
    0b1101010010100010001101011110110100000000010111000000111110011101,
    0b0110000000011101100111000111111000000000000100001011000011010000,
    0b1110100010000111001111101011111111000000001111111111111110110101,
    0b1000000100011111101100101001010110110000111001100010011000011101,
    0b0100110100000001010100000000010000000001000111111011011000011010,
    0b1110011100100100000111110100000101100101010110000010101100010000,

    0b1101100100100110111000101100100000111010010100100001110010001001,
    0b1100100000000101010000010011101000101001000010100000000010100000,
    0b1010000001000101001000101001001100010011010100001011100111100100,
    0b1100100011000011011111001110111010110011010000001111101100010100,
    0b0111001101101100101110100111111110100000110000000001011001100000,
    0b1011110011011101111110100111111110010101000001111110111100010000,
    0b0010000100111111011111100101000100111101001011010001100100011111,
    0b1111001001001110001000010010110100101101111010100001101111110000,
    0b1100011111111010101110101000111111011110010110000110110111100011,
    0b1111110111111100011001110011110101011000001000011110001111011111,

    0b1101101001001011011110100110001010000101000111010111100000101011,
    0b1110000000111101111001101010101011011111110101011001001101111011,
    0b1001110100110001101100000001001111011001010110000011100111111001,
    0b0000101000100011011110010100000000110101101101111011000100100101,
    0b0001011011100111001110111011011011111011000101010001111110000000,
  ];
  
  let mut decoded_rom = Vec::with_capacity(320);
  
  for addr in 0..320 {
    decoded_rom.push(get_word(&rom, addr));
  }
  decoded_rom.try_into().unwrap()
}

pub const WORD_SELECTS: [u11; 16] = [
  u11::new(0b00000000001),  //F0/DPT7
  u11::new(0b00000000010),  //F1/EXPD
  u11::new(0b00000000100),  //F2/LSD1
  u11::new(0b00000001000),  //F3
  u11::new(0b00000010000),  //F4
  u11::new(0b00000100000),  //F5
  u11::new(0b00001000000),  //F6
  u11::new(0b01000000000),  //F9
  u11::new(0b10000000000),  //F10/0V1
  u11::new(0b00000000111),  //OPFGS
  u11::new(0b11000000000),  //MSD1
  u11::new(0b11111111100),  //MANT1
  u11::new(0b11111111100),  //MANT
  u11::new(0b00000000011),  //EXP1
  u11::new(0b00000000011),  //EXP
  u11::new(0b11111111111),  //ALL
];

pub const CONSTANTS: [u4; 16] = [
  u4::new(7), //F0/DPT7
  u4::new(4), //F1/EXPD
  u4::new(1), //F2/LSD1
  u4::new(0),
  u4::new(0),
  u4::new(0),
  u4::new(0),
  u4::new(0),
  u4::new(1), //F10/0V1
  u4::new(0),
  u4::new(1), //MSD1      <- The patent did not document this constant, but the code requires it....
  u4::new(1), //MANT1
  u4::new(0),
  u4::new(1), //EXP1
  u4::new(0),
  u4::new(0),
];

use chips::tms0800::alu::*;
pub const ALU_OPCODES: [Opcode; 32] = [
  Opcode::new(Dest::A, Arg1::A, Oper::Plus, Arg2::B, false),                //0: A = A + B
  Opcode::new(Dest::A, Arg1::A, Oper::Plus, Arg2::K, false),                //1: A = A + K
  Opcode::new(Dest::C, Arg1::A, Oper::Plus, Arg2::K, false),                //2: C = A + K
  Opcode::new(Dest::A, Arg1::None, Oper::Plus, Arg2::B, false),             //3: A = B
  Opcode::new(Dest::C, Arg1::None, Oper::Plus, Arg2::B, false),             //4: C = B
  Opcode::new(Dest::A, Arg1::C, Oper::Plus, Arg2::K, false),                //5: A = C + K
  Opcode::new(Dest::B, Arg1::C, Oper::Plus, Arg2::K, false),                //6: B = C + K
  Opcode::new(Dest::A, Arg1::A, Oper::Minus, Arg2::B, false),               //7: A = A - B
  Opcode::new(Dest::C, Arg1::A, Oper::Minus, Arg2::B, false),               //8: C = A - B
  Opcode::new(Dest::A, Arg1::A, Oper::Minus, Arg2::K, false),               //9: A = A - K
  Opcode::new(Dest::C, Arg1::C, Oper::Minus, Arg2::B, false),               //10: C = C - B
  Opcode::new(Dest::C, Arg1::C, Oper::Minus, Arg2::K, false),               //11: C = C - K
  Opcode::new(Dest::None, Arg1::A, Oper::Minus, Arg2::B, false),            //12: A - B
  Opcode::new(Dest::None, Arg1::A, Oper::Minus, Arg2::K, true),             //13: A - K Hex
  Opcode::new(Dest::None, Arg1::C, Oper::Minus, Arg2::B, false),            //14: C - B
  Opcode::new(Dest::None, Arg1::C, Oper::Minus, Arg2::K, false),            //15: C - K
  Opcode::new(Dest::A, Arg1::None, Oper::Plus, Arg2::K, false),             //16: A = K
  Opcode::new(Dest::B, Arg1::None, Oper::Plus, Arg2::K, false),             //17: B = K
  Opcode::new(Dest::C, Arg1::None, Oper::Plus, Arg2::K, false),             //18: C = K
  Opcode::new(Dest::None, Arg1::None, Oper::ExchangeAB, Arg2::None, false), //19: Exchange A and B
  Opcode::new(Dest::A, Arg1::A, Oper::Shl, Arg2::None, true),               //20: A = A << 4 Hex
  Opcode::new(Dest::B, Arg1::None, Oper::Shl, Arg2::B, true),               //21: B = B << 4 Hex
  Opcode::new(Dest::C, Arg1::C, Oper::Shl, Arg2::None, true),               //22: C = C << 4 Hex
  Opcode::new(Dest::A, Arg1::None, Oper::Shr, Arg2::None, false),           //23: A = A >> 4
  Opcode::new(Dest::B, Arg1::None, Oper::Shr, Arg2::None, false),           //24: B = B >> 4
  Opcode::new(Dest::C, Arg1::None, Oper::Shr, Arg2::None, false),           //25: C = C >> 4
  Opcode::new(Dest::A, Arg1::A, Oper::Wait, Arg2::K, false),                //26: A = A + K Wait
  Opcode::new(Dest::A, Arg1::A, Oper::Plus, Arg2::K, true),                 //27: A = A + K Hex
  Opcode::new(Dest::A, Arg1::A, Oper::Minus, Arg2::K, true),                //28: A = A - K Hex
  Opcode::new(Dest::C, Arg1::C, Oper::Plus, Arg2::K, false),                //29: C = C + K
  Opcode::new(Dest::None, Arg1::None, Oper::Plus, Arg2::None, false),       //30
  Opcode::new(Dest::None, Arg1::None, Oper::Plus, Arg2::None, false),       //31
];