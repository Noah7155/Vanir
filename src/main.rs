use std::io::{stdin, stdout, Read, Stdin};

use termion::{clear, color, cursor, raw::{IntoRawMode, RawTerminal}};

#[derive(Debug)]
struct Board {
    state: [[Option<Peice>; 8]; 8],
    cx: u16,
    cy: u16,
}

#[derive(Copy, Clone, Debug)]
struct Peice {
    color: bool,
    peicetype: PeiceType,
}

#[derive(Copy, Clone, Debug)]
enum PeiceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

fn main() {
    let mut stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut board = Board { 
        state: parsefen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string()), 
        cx: 1,
        cy: 1,
    };
    drawboard(&mut board);

    print!("{}", getkeypress(&mut stdin));
}

fn drawboard(board: &mut Board) {
    print!("{}{}", clear::All, cursor::Goto(board.cx, board.cy));
    let mut rank: usize = 0;
    for i in board.state {
        for (j, peice) in i.iter().enumerate() {
            if peice.is_some() {
                if (j + rank) % 2 == 0 {
                    print!("▐{}{}{}{}{}▌", 
                        color::Bg(color::White), 
                        color::Fg(color::Black), 
                        getpeiceicon(&peice.as_ref().unwrap(), true), 
                        color::Fg(color::Reset),
                        color::Bg(color::Reset),
                    )
                }
                else {
                    print!("{}{}{}",
                        color::Fg(color::White),
                        getpeiceicon(&peice.as_ref().unwrap(), false),
                        color::Fg(color::Reset),
                    )
                }
            }
            else {
                if (j + rank) % 2 == 0 {
                    print!("▐█▌")
                }
                else {
                    print!(" ")
                }
            }
            if j == 7 {
                print!("\r\n");
                rank += 1;
                if (rank+1) % 2 == 0 {
                    print!(" ");
                }
            }
        }
    }
}

fn parsefen(fen: String) -> [[Option<Peice>; 8]; 8] {
    let mut board: [[Option<Peice>; 8]; 8] = [[None; 8]; 8];
    let cleanfen: String = fen.replace("/", "");
    let mut rank: usize = 0;
    let mut file: usize = 0;
    for i in cleanfen.chars() {
        match i {
            'p' => board[rank][file] = Some(Peice { color: false, peicetype: PeiceType::Pawn }),
            'n' => board[rank][file] = Some(Peice { color: false, peicetype: PeiceType::Knight }),
            'b' => board[rank][file] = Some(Peice { color: false, peicetype: PeiceType::Bishop }),
            'r' => board[rank][file] = Some(Peice { color: false, peicetype: PeiceType::Rook }),
            'q' => board[rank][file] = Some(Peice { color: false, peicetype: PeiceType::Queen }),
            'k' => board[rank][file] = Some(Peice { color: false, peicetype: PeiceType::King }),

            'P' => board[rank][file] = Some(Peice { color: true, peicetype: PeiceType::Pawn }),
            'N' => board[rank][file] = Some(Peice { color: true, peicetype: PeiceType::Knight }),
            'B' => board[rank][file] = Some(Peice { color: true, peicetype: PeiceType::Bishop }),
            'R' => board[rank][file] = Some(Peice { color: true, peicetype: PeiceType::Rook }),
            'Q' => board[rank][file] = Some(Peice { color: true, peicetype: PeiceType::Queen }),
            'K' => board[rank][file] = Some(Peice { color: true, peicetype: PeiceType::King }),

            _ => { rank += 1 },
        }
        file += 1;
        if i == '8' { file = 0 }
        if file == 8 { file = 0; rank += 1 };
        if rank == 8 { break };
    }
    return board
}

fn getpeiceicon(peice: &Peice, squarecolor: bool) -> char {
    if squarecolor & peice.color | !squarecolor & !peice.color {
        match peice.peicetype {
            PeiceType::Pawn => return '♙',
            PeiceType::Knight => return '♘',
            PeiceType::Bishop => return '♗',
            PeiceType::Rook => return '♖',
            PeiceType::Queen => return '♕',
            PeiceType::King => return '♔',
        }
    }
    else {
        match peice.peicetype {
            PeiceType::Pawn => return '♟',
            PeiceType::Knight => return '♞',
            PeiceType::Bishop => return '♝',
            PeiceType::Rook => return '♜',
            PeiceType::Queen => return '♛',
            PeiceType::King => return '♚',
        }
    }
}

fn movepeice(board: &mut Board, start: (usize, usize), target: (usize, usize)) {
    board.state[target.0][target.1] = board.state[start.0][start.1];
    board.state[start.0][start.1] = None;

} 

fn getkeypress(stdin: &mut Stdin) -> u8 {
    for c in stdin.bytes() {
        return c.expect("KeyError");
    }
    return 0;
}
