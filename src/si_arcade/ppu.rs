use std::cell::RefCell;
use std::rc::Rc;

use crate::binary_lib::get_bit;

use super::mmu::Mmu;

pub const SCREEN_FREQUENCY: usize = 60;
pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 224;

pub const GAME_TEXTURE_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT * 3;
pub const GAME_OVERLAY_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT * 4;

const OVERLAY_TEXTURE_SV: &[u8] = include_bytes!("../../game_overlays/space_invaders_overlay_1_SV.png");
const OVERLAY_TEXTURE_TV: &[u8] = include_bytes!("../../game_overlays/space_invaders_overlay_2_TV.png");
const OVERLAY_TEXTURE_CV: &[u8] = include_bytes!("../../game_overlays/space_invaders_overlay_3_CV.png");

pub struct Ppu {
    mmu: Rc<RefCell<Mmu>>,
    screen: [u8; GAME_TEXTURE_SIZE],
    overlay: [u8; GAME_OVERLAY_SIZE],
}

impl Ppu {
    pub fn new(mmu: &Rc<RefCell<Mmu>>, display_mode: String) -> Ppu {
        let overlay_image = match display_mode.as_str() {
            "SV" => OVERLAY_TEXTURE_SV,
            "TV" => OVERLAY_TEXTURE_TV,
            "CV" => OVERLAY_TEXTURE_CV,
            _ => OVERLAY_TEXTURE_SV,
        };

        Ppu {
            mmu: Rc::clone(mmu),
            screen: [0; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
            overlay: image::load_from_memory(overlay_image)
                .unwrap()
                .into_bytes()
                .as_slice()
                .try_into()
                .unwrap(),
        }
    }

    pub fn clock(&mut self) {
        let mut index: usize = 0;
        for data in self.mmu.borrow().get_vram() {
            for bit in 0..8 {
                let color: u8 = if get_bit(*data, bit) { 0xFF } else { 0 };
                self.screen[index] = color;
                self.screen[index + 1] = color;
                self.screen[index + 2] = color;
                index += 3;
            }
        }
    }

    pub fn get_screen(&self) -> &[u8; GAME_TEXTURE_SIZE] {
        &self.screen
    }

    pub fn get_overlay(&self) -> &[u8; GAME_OVERLAY_SIZE] {
        &self.overlay
    }
}
