use crate::piece::{Color, Piece};

pub struct Player {
    name: String,
    color: Color,
    acquired_pieces: Vec<Piece>,
}

impl Player {
    pub fn new(name: String, color: Color) -> Self {
        Player {
            name,
            color,
            acquired_pieces: vec![],
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn get_acquired_pieces(&self) -> &Vec<Piece> {
        &self.acquired_pieces
    }

    pub fn add_acquired_piece(&mut self, piece: Piece) {
        self.acquired_pieces.push(piece);
    }
}
