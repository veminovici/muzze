use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Accidental {
    Natural = 0,
    Reset = 1,

    Flat = 2,
    DoubleFlat = 3,

    Sharp = 8,
    DoubleSharp = 9,
}

impl Display for Accidental {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Accidental::Natural => write!(f, ""),
            Accidental::Reset => write!(f, "♮"),
            Accidental::Flat => write!(f, "♭"),
            Accidental::DoubleFlat => write!(f, "♭♭"),
            Accidental::Sharp => write!(f, "♯"),
            Accidental::DoubleSharp => write!(f, "♯♯"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(Accidental::Natural.to_string(), "");
        assert_eq!(Accidental::Reset.to_string(), "♮");
        assert_eq!(Accidental::Flat.to_string(), "♭");
        assert_eq!(Accidental::DoubleFlat.to_string(), "♭♭");
        assert_eq!(Accidental::Sharp.to_string(), "♯");
        assert_eq!(Accidental::DoubleSharp.to_string(), "♯♯");
    }
}
