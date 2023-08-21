use crate::error::Error;

/// Event kind.
#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum Kind {
    Metadata = 0,
    TextNote = 1,
}

impl TryFrom<i32> for Kind {
    type Error = Error;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == Kind::Metadata as i32 => Ok(Kind::Metadata),
            x if x == Kind::TextNote as i32 => Ok(Kind::TextNote),
            x => Err(Error::Custom(format!("did not find a kind matching \"{x}\""))),
        }
    }
}
