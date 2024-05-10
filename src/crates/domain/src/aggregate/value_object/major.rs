#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Major {
    ComputerScience,
    Economics,
    Law,
    Art,
    Music,
    Other,
}

impl std::convert::From<&str> for Major {
    fn from(value: &str) -> Self {
        match value {
            "ComputerScience" => Major::ComputerScience,
            "Economics" => Major::Economics,
            "Law" => Major::Law,
            "Art" => Major::Art,
            "Music" => Major::Music,
            _ => Major::Other,
        }
    }
}

impl std::convert::From<Major> for String {
    fn from(value: Major) -> Self {
        match value {
            Major::ComputerScience => "ComputerScience",
            Major::Economics => "Economics",
            Major::Law => "Law",
            Major::Art => "Art",
            Major::Music => "Music",
            Major::Other => "Other",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        for (v, s) in [
            (Major::ComputerScience, "ComputerScience"),
            (Major::Economics, "Economics"),
            (Major::Law, "Law"),
            (Major::Art, "Art"),
            (Major::Music, "Music"),
            (Major::Other, "Other"),
        ] {
            assert_eq!(Major::from(s), v);
            assert_eq!(String::from(v), s);
        }
    }
}
