enum Chapter {
    Prologue, City(Side), Site(Side), Resort(Side), Ridge(Side), Temple(Side), Reflection(Side), Summit(Side), Epilogue, Core(Side), Farewell
}

impl Chapter {
    pub fn to_string(&self) -> &str {
        match self {
            Self::Prologue | Self::Epilogue | Self::Farewell => self.long_name(),
            chapter(side) => chapter.
        }
    }

    pub fn short_name()

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

enum Side {
    A, B, C
}

impl Side {
    pub fn to_string(&self) -> &str {
        match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
        }
    }
}

const ANY_PERCENT_ROUTE: [Chapter; 8] = [
    Chapter::Prologue,
    Chapter::City(Side::A),
    Chapter::Site(Side::A),
    Chapter::Resort(Side::A),
    Chapter::Ridge(Side::A),
    Chapter::Temple(Side::A),
    Chapter::Reflection(Side::A),
    Chapter::Summit(Side::A),
];

fn main() {
    println!("Hello, world!");
}
