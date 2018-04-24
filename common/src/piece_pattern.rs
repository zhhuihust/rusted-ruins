//! Tile pattern will be defined by surround 8 tiles.
//! TilePatternFlag helps to search appropriate pattern.

use array2d::*;
use basic::{PIECE_SIZE, PIECE_SIZE_I};
use obj::ImgObject;
use objholder::{TileIdx, WallIdx};

/// Represents 4 pieces pattern of tile images
#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct PiecePattern {
    pub top_left: u8,
    pub top_right: u8,
    pub bottom_left: u8,
    pub bottom_right: u8,
}

impl Default for PiecePattern {
    fn default() -> PiecePattern {
        PiecePattern::SURROUNDED
    }
}

impl PiecePattern {
    pub fn is_empty(self) -> bool {
        self == Self::EMPTY
    }

    pub const SURROUNDED: PiecePattern = PiecePattern {
        top_left: 0,
        top_right: 0,
        bottom_left: 0,
        bottom_right: 0,
    };

    /// Represents that the tile (or wall, etc) is empty
    pub const EMPTY: PiecePattern = PiecePattern {
        top_left: 0xFF,
        top_right: 0xFF,
        bottom_left: 0xFF,
        bottom_right: 0xFF,
    };
}

/// TileIdx or WallIdx with piece pattern
#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct IdxWithPiecePattern<T> {
    pub idx: T,
    pub piece_pattern: PiecePattern,
}

impl<T> IdxWithPiecePattern<T> {
    pub fn is_empty(&self) -> bool {
        self.piece_pattern.is_empty()
    }
}

impl<T> Default for IdxWithPiecePattern<T> where T: Default {
    fn default() -> IdxWithPiecePattern<T> {
        IdxWithPiecePattern {
            idx: T::default(),
            piece_pattern: PiecePattern::EMPTY,
        }
    }
}

pub type TileIdxPP = IdxWithPiecePattern<TileIdx>;
pub type WallIdxPP = IdxWithPiecePattern<WallIdx>;
pub type ConvertedIdxPP = IdxWithPiecePattern<u32>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PiecePatternFlags(pub u8);

impl PiecePatternFlags {
    pub fn new() -> PiecePatternFlags {
        PiecePatternFlags(0)
    }

    pub fn set(&mut self, dir: Direction, is_same_tile: bool) {
        let flag = match dir {
            Direction::N  => Self::N,
            Direction::S  => Self::S,
            Direction::E  => Self::E,
            Direction::W  => Self::W,
            Direction::NE => Self::NE,
            Direction::NW => Self::NW,
            Direction::SE => Self::SE,
            Direction::SW => Self::SW,
            Direction::NONE => { return; }
        };
        if is_same_tile {
            self.0 |= flag;
        } else {
            self.0 &= !flag;
        }
    }

    pub fn to_piece_pattern(self) -> PiecePattern {
        PiecePattern {
            top_left: get_corner_piece_pattern(
                self.0 & Self::N,
                self.0 & Self::NW,
                self.0 & Self::W),
            top_right: get_corner_piece_pattern(
                self.0 & Self::N,
                self.0 & Self::NE,
                self.0 & Self::E),
            bottom_left: get_corner_piece_pattern(
                self.0 & Self::S,
                self.0 & Self::SW,
                self.0 & Self::W),
            bottom_right: get_corner_piece_pattern(
                self.0 & Self::S,
                self.0 & Self::SE,
                self.0 & Self::E),
        }
    }

    const E:  u8 = 0b00000001;
    const SE: u8 = 0b00000010;
    const S:  u8 = 0b00000100;
    const SW: u8 = 0b00001000;
    const W:  u8 = 0b00010000;
    const NW: u8 = 0b00100000;
    const N:  u8 = 0b01000000;
    const NE: u8 = 0b10000000;
}

fn get_corner_piece_pattern(ns: u8, between: u8, ew: u8) -> u8 {
    let ns = ns != 0;
    let between = between != 0;
    let ew = ew != 0;
    
    match (ns, between, ew) {
        (false, false, false) => 3,
        (false, false, true ) => 1,
        (false, true , false) => 3,
        (false, true , true ) => 1,
        (true , false, false) => 2,
        (true , false, true ) => 4,
        (true , true , false) => 2,
        (true , true , true ) => 0,
    }
}

pub trait PieceImgObject: ImgObject {
    fn n_pattern(&self) -> u8;
    
    /// Get rect of the specified piece
    fn piece_rect(&self, i_pattern: u8, i_piece: u32, i_anim_frame: u32) -> (i32, i32, u32, u32) {
        let n_frame = self.get_img().n_frame;
        let n_pattern = self.n_pattern() as u32;
        let n_anim_frame = n_frame / n_pattern;
        let img_rect = self.img_rect_nth(n_anim_frame * i_pattern as u32 + i_anim_frame);

        match i_piece {
            0 => ( // Top left piece
                img_rect.0,
                img_rect.1,
                PIECE_SIZE,
                PIECE_SIZE),
            1 => ( // Top right piece
                img_rect.0 + PIECE_SIZE_I,
                img_rect.1,
                PIECE_SIZE,
                PIECE_SIZE),
            2 => ( // Bottom left piece
                img_rect.0,
                img_rect.1 + PIECE_SIZE_I,
                PIECE_SIZE,
                img_rect.3 - PIECE_SIZE),
            3 => ( // Bottom right piece
                img_rect.0 + PIECE_SIZE_I,
                img_rect.1 + PIECE_SIZE_I,
                PIECE_SIZE,
                img_rect.3 - PIECE_SIZE),
            _ => unreachable!(),
        }
    }
}

impl PieceImgObject for ::obj::TileObject {
    fn n_pattern(&self) -> u8 {
        self.n_pattern
    }
}

impl PieceImgObject for ::obj::WallObject {
    fn n_pattern(&self) -> u8 {
        self.n_pattern
    }
}

