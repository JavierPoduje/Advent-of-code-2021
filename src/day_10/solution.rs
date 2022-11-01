use std::collections::VecDeque;

use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (u64, u64) {
    let input = read_one_per_line::<String>("./src/day_10/input.txt").unwrap();
    let chunks: Vec<Chunk> = input
        .into_iter()
        .filter(|row| !row.is_empty())
        .map(Chunk::new)
        .collect();

    let mut part1 = 0;
    let mut scores: Vec<u64> = Vec::new();

    for chunk in chunks {
        let corrupted_char = chunk.first_corrupted_char();
        match corrupted_char {
            // part 1
            Some(char) => part1 += char.penalty(),
            // part 2
            None => {
                let closing_chars = chunk.closing_chars();
                let mut score = 0;
                for closing_char in closing_chars {
                    score *= 5;
                    score += closing_char.autocomplete_score();
                }
                scores.push(score);
            }
        }
    }

    scores.sort();

    (part1, scores[scores.len() / 2])
}

struct Chunk {
    chars: Vec<Char>,
}

impl Chunk {
    pub fn new(raw_line: String) -> Self {
        let chars = raw_line.chars().map(|char| Char::new(char)).collect();
        Self { chars }
    }

    pub fn closing_chars(&self) -> Vec<Char> {
        let mut stack: VecDeque<&Char> = VecDeque::new();

        for char in &self.chars {
            if char.is_open() {
                stack.push_back(char);
            } else {
                stack.pop_back().unwrap();
            }
        }

        let mut chars: Vec<Char> = Vec::new();
        for char in stack.into_iter().rev() {
            chars.push(char.mirror());
        }

        chars
    }

    pub fn first_corrupted_char(&self) -> Option<&Char> {
        let mut stack: VecDeque<&Char> = VecDeque::new();

        for char in &self.chars {
            if char.is_open() {
                stack.push_back(char);
            } else {
                let open = stack.pop_back().unwrap();
                if open != &char.mirror() {
                    return Some(char);
                }
            }
        }

        None
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Char {
    // open
    Round,
    Rect,
    Curly,
    Lt,
    // close
    RoundClose,
    RectClose,
    CurlyClose,
    LtClose,
}

impl Char {
    pub fn new(char: char) -> Self {
        match char {
            '(' => Char::Round,
            '[' => Char::Rect,
            '{' => Char::Curly,
            '<' => Char::Lt,
            ')' => Char::RoundClose,
            ']' => Char::RectClose,
            '}' => Char::CurlyClose,
            '>' => Char::LtClose,
            _ => unreachable!(),
        }
    }

    pub fn mirror(&self) -> Self {
        match self {
            Char::Round => Char::RoundClose,
            Char::Rect => Char::RectClose,
            Char::Curly => Char::CurlyClose,
            Char::Lt => Char::LtClose,
            Char::RoundClose => Char::Round,
            Char::RectClose => Char::Rect,
            Char::CurlyClose => Char::Curly,
            Char::LtClose => Char::Lt,
        }
    }

    pub fn is_open(&self) -> bool {
        match self {
            Char::Round | Char::Rect | Char::Curly | Char::Lt => true,
            _ => false,
        }
    }

    pub fn penalty(&self) -> u64 {
        match self {
            Char::Round | Char::RoundClose => 3,
            Char::Rect | Char::RectClose => 57,
            Char::Curly | Char::CurlyClose => 1197,
            Char::Lt | Char::LtClose => 25137,
        }
    }

    pub fn autocomplete_score(&self) -> u64 {
        match self {
            Char::Round | Char::RoundClose => 1,
            Char::Rect | Char::RectClose => 2,
            Char::Curly | Char::CurlyClose => 3,
            Char::Lt | Char::LtClose => 4,
        }
    }
}
