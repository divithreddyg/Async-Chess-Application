use crate::*;
use std::fmt::{Display, Result};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Piece {
    King(Color),
    Queen(Color),
    Rook(Color),
    Bishop(Color),
    Knight(Color),
    Pawn(Color),
}
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn other(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

impl Piece {
    pub fn get_color(&self) -> &Color {
        match self {
            Piece::King(color) => color,
            Piece::Queen(color) => color,
            Piece::Rook(color) => color,
            Piece::Bishop(color) => color,
            Piece::Knight(color) => color,
            Piece::Pawn(color) => color,
        }
    }


    pub fn _is_some_piece(&self, piece: &Piece) -> bool {
        match self {
            piece => true,
            _ => false,
        }
    }

    pub fn get_string_repr(&self) -> String {
        let color_str = match self.get_color() {
            Color::Black => "B",
            Color::White => "W",
        };

        match self {
            Piece::King(color) => format!("{}", if *color == Color::White {"K"} else {"k"}),
            Piece::Queen(color) => format!("{}", if *color == Color::White {"Q"} else {"q"}),
            Piece::Rook(color) => format!("{}", if *color == Color::White {"R"} else {"r"}),
            Piece::Bishop(color) => format!("{}", if *color == Color::White {"B"} else {"b"}),
            Piece::Knight(color) => format!("{}", if *color == Color::White {"N"} else {"n"}),
            Piece::Pawn(color) => format!("{}", if *color == Color::White {"P"} else {"p"}),
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Piece::King(color) => match color {
                Color::White => write!(f, "♔"),
                Color::Black => write!(f, "{}", String::from("♚").black()),
            },
            Piece::Queen(color) => match color {
                Color::White => write!(f, "♕"),
                Color::Black => write!(f, "{}", String::from("♛").black()),
            },
            Piece::Rook(color) => match color {
                Color::White => write!(f, "♖"),
                Color::Black => write!(f, "{}", String::from("♜").black()),
            },
            Piece::Bishop(color) => match color {
                Color::White => write!(f, "♗"),
                Color::Black => write!(f, "{}", String::from("♝").black()),
            },
            Piece::Knight(color) => match color {
                Color::White => write!(f, "♘"),
                Color::Black => write!(f, "{}", String::from("♞").black()),
            },
            Piece::Pawn(color) => match color {
                Color::White => write!(f, "♙"),
                Color::Black => write!(f, "{}", String::from("♟").black()),
            },
        }
    }
}
