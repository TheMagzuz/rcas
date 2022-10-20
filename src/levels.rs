use anyhow::anyhow;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Chapter {
    Prologue, City(Side), Site(Side), Resort(Side), Ridge(Side), Temple(Side), Reflection(Side), Summit(Side), Epilogue, Core(Side), Farewell
}

impl Chapter {
    pub fn from_index(index: u8, side: Side) -> anyhow::Result<Self> {
        match index {
            0 => Ok(Self::Prologue),
            1 => Ok(Self::City(side)),
            2 => Ok(Self::Site(side)),
            3 => Ok(Self::Resort(side)),
            4 => Ok(Self::Ridge(side)),
            5 => Ok(Self::Temple(side)),
            6 => Ok(Self::Reflection(side)),
            7 => Ok(Self::Summit(side)),
            8 => Ok(Self::Epilogue),
            9 => Ok(Self::Core(side)),
            10 => Ok(Self::Farewell),
            _ => Err(anyhow!("Invalid chapter index: {}", index)),
        }
    }
    
    pub fn to_string(&self) -> String {
        match self {
            Self::Prologue | Self::Epilogue | Self::Farewell => self.long_name().to_owned(),
            chapter @ (Self::City(side) | Self::Site(side) | Self::Resort(side) | Self::Ridge(side) | Self::Temple(side) | Self::Reflection(side) | Self::Summit(side) | Self::Core(side)) => chapter.short_name().to_owned() + side.to_string(),
        }
    }

    pub fn short_name(&self) -> &str {
        match self {
            Self::Prologue => "Prologue",
            Self::City(_) => "1",
            Self::Site(_) => "2",
            Self::Resort(_) => "3",
            Self::Ridge(_) => "4",
            Self::Temple(_) => "5",
            Self::Reflection(_) => "6",
            Self::Summit(_) => "7",
            Self::Epilogue => "Epilogue",
            Self::Core(_) => "8",
            Self::Farewell => "9"
        }
    }

    pub fn long_name(&self) -> &str {
        match self {
            Self::Prologue => "Prologue",
            Self::City(_) => "Forsaken City",
            Self::Site(_) => "Old Site",
            Self::Resort(_) => "Celestial Resort",
            Self::Ridge(_) => "Golden Ridge",
            Self::Temple(_) => "Mirror Temple",
            Self::Reflection(_) => "Reflection",
            Self::Summit(_) => "The Summit",
            Self::Epilogue => "Epilogue",
            Self::Core(_) => "Core",
            Self::Farewell => "Farewell"
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Side {
    A, B, C
}

impl Side {
    pub fn from_index(index: u8) -> anyhow::Result<Self> {
        match index {
            0 => Ok(Self::A),
            1 => Ok(Self::B),
            2 => Ok(Self::C),
            _ => Err(anyhow!("Invalid side index: {}", index)),
        }
    }
    pub fn to_string(&self) -> &str {
        match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
        }
    }
}

pub const ANY_PERCENT_ROUTE: [Chapter; 8] = [
    Chapter::Prologue,
    Chapter::City(Side::A),
    Chapter::Site(Side::A),
    Chapter::Resort(Side::A),
    Chapter::Ridge(Side::A),
    Chapter::Temple(Side::A),
    Chapter::Reflection(Side::A),
    Chapter::Summit(Side::A),
];
