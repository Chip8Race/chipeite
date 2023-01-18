pub mod framebuffer;
pub mod key_state;
pub mod registers;
pub mod stack;

use crate::video::Video;

use self::{framebuffer::FrameBuffer, key_state::KeyState, registers::Registers, stack::Stack};

const FOUR_K: usize = 4 * 1024;

pub struct Memory {
    pub framebuffer: FrameBuffer,
    pub ram: [u8; FOUR_K],
    pub registers: Registers,
    pub stack: Stack,
    pub sound_timer: u8,
    pub delay_timer: u8,
    pub key_state: KeyState,
    pub video: Video,
}

impl Memory {
    pub fn new(program: &[u8], video: Video) -> Self {
        let mut this = Self {
            framebuffer: FrameBuffer::default(),
            ram: [0; FOUR_K],
            registers: Registers::default(),
            stack: Stack::default(),
            sound_timer: 0,
            delay_timer: 0,
            key_state: KeyState::default(),
            video,
        };
        this.ram[0..80].copy_from_slice(&FONT_SPEC);
        let program_len = program.len();
        this.ram[0x200..(0x200 + program_len)].copy_from_slice(program);
        this
    }
}

const FONT_SPEC: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
