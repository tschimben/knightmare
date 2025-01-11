use std::fmt::Display;

/// A file on the chess board (`A`-`H`)
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum File {
    FileA = 0,
    FileB = 1,
    FileC = 2,
    FileD = 3,
    FileE = 4,
    FileF = 5,
    FileG = 6,
    FileH = 7,
}

impl File {
    /// Tries to get a file enum from a [u8] between `0` and `7`
    pub fn from_u8(file: u8) -> Option<Self> {
        match file {
            0 => Some(Self::FileA),
            1 => Some(Self::FileB),
            2 => Some(Self::FileC),
            3 => Some(Self::FileD),
            4 => Some(Self::FileE),
            5 => Some(Self::FileF),
            6 => Some(Self::FileG),
            7 => Some(Self::FileH),
            _ => None,
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            File::FileA => 'A',
            File::FileB => 'B',
            File::FileC => 'C',
            File::FileD => 'D',
            File::FileE => 'E',
            File::FileF => 'F',
            File::FileG => 'G',
            File::FileH => 'H',
        };

        write!(f, "{c}")
    }
}

/// A rank on the chessboard (`1`-`8`)
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Rank {
    Rank1 = 0,
    Rank2 = 1,
    Rank3 = 2,
    Rank4 = 3,
    Rank5 = 4,
    Rank6 = 5,
    Rank7 = 6,
    Rank8 = 7,
}

impl Rank {
    /// Tries to get a rank enum from a [u8] between `0` and `7`
    pub fn from_u8(rank: u8) -> Option<Self> {
        match rank {
            0 => Some(Self::Rank1),
            1 => Some(Self::Rank2),
            2 => Some(Self::Rank3),
            3 => Some(Self::Rank4),
            4 => Some(Self::Rank5),
            5 => Some(Self::Rank6),
            6 => Some(Self::Rank7),
            7 => Some(Self::Rank8),
            _ => None,
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Rank::Rank1 => '1',
            Rank::Rank2 => '2',
            Rank::Rank3 => '3',
            Rank::Rank4 => '4',
            Rank::Rank5 => '5',
            Rank::Rank6 => '6',
            Rank::Rank7 => '7',
            Rank::Rank8 => '8',
        };

        write!(f, "{c}")
    }
}

/// A coordinate on the chess board made up from [File]s and [Rank]s
#[derive(Copy, Clone)]
pub struct Coordinate {
    pub file: File,
    pub rank: Rank,
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}

impl Coordinate {
    /// Constructs a coordinate starting from `A1`
    /// using files and ranks that reach from `0`(`A/1`) to `7`(`H/8`)
    /// # Arguments
    /// * `file` - The file
    /// * `rank` - The rank
    pub fn from_u8s(file: u8, rank: u8) -> Option<Coordinate> {
        Some(Self {
            file: File::from_u8(file)?,
            rank: Rank::from_u8(rank)?,
        })
    }
}
