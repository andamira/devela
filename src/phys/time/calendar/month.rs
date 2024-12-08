// devela::phys::time::calendar::month
//
//!
//

use crate::{Display, FmtResult, Formatter, FromStr};
#[allow(clippy::enum_glob_use)]
use Month::*;

/// The months of the year.
#[repr(u8)]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Month {
    January = 0,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    /// The number of months in a year.
    pub const COUNT: usize = 12;

    /// Returns the length in days of the current month, taking into account
    /// whether it's a `leap` year, for february.
    #[allow(clippy::len_without_is_empty)]
    pub const fn len(self, leap: bool) -> u8 {
        match self {
            January => 31,
            February => 28 + leap as u8,
            March => 31,
            April => 30,
            May => 31,
            June => 30,
            July => 31,
            August => 31,
            September => 30,
            October => 31,
            November => 30,
            December => 31,
        }
    }

    /// Returns the previous month.
    pub const fn previous(self) -> Month {
        self.previous_nth(1)
    }

    /// Returns the previous `nth` month.
    pub const fn previous_nth(self, nth: usize) -> Month {
        Self::from_index_unchecked(self.index().wrapping_sub(nth) % Self::COUNT)
    }

    /// Returns the next month.
    pub const fn next(self) -> Month {
        self.next_nth(1)
    }

    /// Returns the next `nth` month.
    pub const fn next_nth(self, nth: usize) -> Month {
        Self::from_index_unchecked(self.index().wrapping_add(nth) % Self::COUNT)
    }

    /* numbers */

    /// Returns the Month number from `January=1` to `December=12`.
    pub const fn number(self) -> u8 {
        self.index() as u8 + 1
    }

    /// Returns the Month index from `January=0` to `December=11`.
    pub const fn index(self) -> usize {
        self as _
    }

    /// Returns a `Month` from its counting number, from `January=1` to `December=12`.
    ///
    /// # Errors
    /// `if n < 1 || n > 12`
    pub const fn from_number(n: u8) -> Result<Month, &'static str> {
        match n {
            1 => Ok(January),
            2 => Ok(February),
            3 => Ok(March),
            4 => Ok(April),
            5 => Ok(May),
            6 => Ok(June),
            7 => Ok(July),
            8 => Ok(August),
            9 => Ok(September),
            10 => Ok(October),
            11 => Ok(November),
            12 => Ok(December),
            _ => Err("The month number must be between 1 and 12."),
        }
    }

    /// Returns a `Month` from its index, from `January=0` to `December=11`.
    ///
    /// # Errors
    /// `if index > 11`
    pub const fn from_index(index: usize) -> Result<Month, &'static str> {
        match index {
            0 => Ok(January),
            1 => Ok(February),
            2 => Ok(March),
            3 => Ok(April),
            4 => Ok(May),
            5 => Ok(June),
            6 => Ok(July),
            7 => Ok(August),
            8 => Ok(September),
            9 => Ok(October),
            10 => Ok(November),
            11 => Ok(December),
            _ => Err("The month index must be between 0 and 11."),
        }
    }

    /// Returns a `Month` from its index, from `January=0` to `December=11`.
    ///
    /// # Panics
    /// `if index > 11`
    pub const fn from_index_unchecked(index: usize) -> Self {
        match index {
            0 => January,
            1 => February,
            2 => March,
            3 => April,
            4 => May,
            5 => June,
            6 => July,
            7 => August,
            8 => September,
            9 => October,
            10 => November,
            11 => December,
            _ => panic!("The month index must be between 0 and 11."),
        }
    }
}

/// # abbreviations & representations
#[allow(missing_docs, non_upper_case_globals)]
impl Month {
    /// Returns the 3-letter abbreviated month name, in ASCII, UpperCamelCase.
    pub const fn abbr3(self) -> &'static str {
        match self {
            January => "Jan",
            February => "Feb",
            March => "Mar",
            April => "Apr",
            May => "May",
            June => "Jun",
            July => "Jul",
            August => "Aug",
            September => "Sep",
            October => "Oct",
            November => "Nov",
            December => "Dec",
        }
    }

    pub const Jan: Month = Month::January;
    pub const Feb: Month = Month::February;
    pub const Mar: Month = Month::March;
    pub const Apr: Month = Month::April;
    pub const May: Month = Month::May;
    pub const Jun: Month = Month::June;
    pub const Jul: Month = Month::July;
    pub const Aug: Month = Month::August;
    pub const Sep: Month = Month::September;
    pub const Oct: Month = Month::October;
    pub const Nov: Month = Month::November;
    pub const Dec: Month = Month::December;

    /// Returns the 2-letter abbreviated month name, in ASCII, UPPERCASE.
    pub const fn abbr2(self) -> &'static str {
        match self {
            January => "JA",
            February => "FE",
            March => "MR",
            April => "AP",
            May => "MY",
            June => "JN",
            July => "JL",
            August => "AU",
            September => "SE",
            October => "OC",
            November => "NV",
            December => "DE",
        }
    }

    pub const JA: Month = Month::January;
    pub const FE: Month = Month::February;
    pub const MR: Month = Month::March;
    pub const AP: Month = Month::April;
    pub const MY: Month = Month::May;
    pub const JN: Month = Month::June;
    pub const JL: Month = Month::July;
    pub const AU: Month = Month::August;
    pub const SE: Month = Month::September;
    pub const OC: Month = Month::October;
    pub const NV: Month = Month::November;
    pub const DE: Month = Month::December;

    /// Returns the 1-letter abbreviated month name, in ASCII, UPPERCASE.
    pub const fn abbr1(self) -> &'static str {
        match self {
            January => "J",
            February => "F",
            March => "R",
            April => "P",
            May => "Y",
            June => "N",
            July => "L",
            August => "U",
            September => "S",
            October => "O",
            November => "N",
            December => "D",
        }
    }

    pub const J: Month = Month::January;
    pub const F: Month = Month::February;
    pub const R: Month = Month::March;
    pub const P: Month = Month::April;
    pub const Y: Month = Month::May;
    pub const N: Month = Month::June;
    pub const L: Month = Month::July;
    pub const U: Month = Month::August;
    pub const S: Month = Month::September;
    pub const O: Month = Month::October;
    pub const V: Month = Month::November;
    pub const D: Month = Month::December;

    /// Returns an emoji associated to each month.
    ///
    /// These are: 🌺, 🐉, 🍀, 🐰, 🌼, 🐟, 🌞, 🍂, 🎃, 🦉, 🍁, 🎄.
    ///
    /// Hibiscus, Dragon, Four Leaf Clover, Rabbit, Blossom, Fish, Sun with Face, Fallen Leaf,
    /// Jack-O-Lantern, Owl, Maple Leaf and Christmas Tree.
    pub const fn emoji(self) -> char {
        match self {
            // Hibiscus.
            January => '🌺',
            // Dragon.
            February => '🐉',
            // Four Leaf Clover.
            March => '🍀',
            // Rabbit.
            April => '🐰',
            // Blossom.
            May => '🌼',
            // Fish.
            June => '🐟',
            // Sun with Face.
            July => '🌞',
            // Fallen LEaf.
            August => '🍂',
            // Jack-O-Lantern.
            September => '🎃',
            // Owl.
            October => '🦉',
            // Maple Leaf.
            November => '🍁',
            // Christmas Tree.
            December => '🎄',
        }
    }

    /// Returns the main zodiac symbol, associated to the start of the month.
    ///
    /// These are: ♑, ♒, ♓, ♈, ♉, ♊, ♋, ♌, ♍, ♎, ♏, ♐.
    ///
    /// Capricorn, Aquarius, Pisces, Aries, Taurus, Gemini, Cancer, Leo, Virgo,
    /// Libra, Scorpio, Sagittarius.
    ///
    /// # Examples
    /// ```
    /// # use devela::Month;
    /// assert_eq![Month::July.zodiac_start(), '♋'];
    /// ```
    pub const fn zodiac_start(self) -> char {
        match self {
            // Capricorn
            January => '♑',
            // Aquarius.
            February => '♒',
            // Pisces.
            March => '♓',
            // Aries.
            April => '♈',
            // Taurus.
            May => '♉',
            // Gemini.
            June => '♊',
            // Cancer.
            July => '♋',
            // Leo.
            August => '♌',
            // Virgo.
            September => '♍',
            // Libra.
            October => '♎',
            // Scorpio.
            November => '♏',
            // Sagittarius.
            December => '♐',
        }
    }

    /// Returns the main zodiac name, associated to the start of the month.
    ///
    /// These are: Capricorn, Aquarius, Pisces, Aries, Taurus, Gemini, Cancer,
    /// Leo, Virgo, Libra, Scorpio, Sagittarius.
    ///
    /// # Examples
    /// ```
    /// # use devela::Month;
    /// assert_eq![Month::July.zodiac_start_name(), "Cancer"];
    /// ```
    pub const fn zodiac_start_name(self) -> &'static str {
        match self {
            January => "Capricorn",
            February => "Aquarius",
            March => "Pisces",
            April => "Aries",
            May => "Taurus",
            June => "Gemini",
            July => "Cancer",
            August => "Leo",
            September => "Virgo",
            October => "Libra",
            November => "Scorpio",
            December => "Sagittarius",
        }
    }

    /// Returns the secondary zodiac symbol, associated to the end of the month.
    ///
    /// These are: ♒, ♓, ♈, ♉, ♊, ♋, ♌, ♍, ♎, ♏, ♐, ♑.
    ///
    /// # Examples
    /// ```
    /// # use devela::Month;
    /// assert_eq![Month::July.zodiac_end(), '♌'];
    /// ```
    pub const fn zodiac_end(self) -> char {
        self.next().zodiac_start()
    }

    /// Returns the secondary zodiac name, associated to the end of the month.
    ///
    /// These are: Aquarius, Pisces, Aries, Taurus, Gemini, Cancer, Leo, Virgo,
    /// Libra, Scorpio, Sagittarius, Capricorn.
    ///
    /// # Examples
    /// ```
    /// # use devela::Month;
    /// assert_eq![Month::July.zodiac_end_name(), "Leo"];
    /// ```
    pub const fn zodiac_end_name(self) -> &'static str {
        self.next().zodiac_start_name()
    }
}

impl Display for Month {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        f.write_str(match self {
            January => "January",
            February => "February",
            March => "March",
            April => "April",
            May => "May",
            June => "June",
            July => "July",
            August => "August",
            September => "September",
            October => "October",
            November => "November",
            December => "December",
        })
    }
}

impl From<Month> for u8 {
    fn from(month: Month) -> Self {
        month as _
    }
}

/// Returns a `Month` from a string containing either the full month name,
/// or any of the month ASCII abbreviations.
impl FromStr for Month {
    type Err = &'static str;

    #[rustfmt::skip]
    fn from_str(s: &str) -> Result<Month, Self::Err> {
        if s.eq_ignore_ascii_case("January") { Ok(January)
        } else if s.eq_ignore_ascii_case("February") { Ok(February)
        } else if s.eq_ignore_ascii_case("March") { Ok(March)
        } else if s.eq_ignore_ascii_case("April") { Ok(April)
        } else if s.eq_ignore_ascii_case("May") { Ok(May)
        } else if s.eq_ignore_ascii_case("June") { Ok(June)
        } else if s.eq_ignore_ascii_case("July") { Ok(July)
        } else if s.eq_ignore_ascii_case("August") { Ok(August)
        } else if s.eq_ignore_ascii_case("September") { Ok(September)
        } else if s.eq_ignore_ascii_case("October") { Ok(October)
        } else if s.eq_ignore_ascii_case("November") { Ok(November)
        } else if s.eq_ignore_ascii_case("December") { Ok(December)
        //
        } else if s.eq_ignore_ascii_case("Jan") { Ok(January)
        } else if s.eq_ignore_ascii_case("Feb") { Ok(February)
        } else if s.eq_ignore_ascii_case("Mar") { Ok(March)
        } else if s.eq_ignore_ascii_case("Apr") { Ok(April)
        // } else if s.eq_ignore_ascii_case("May") { Ok(May) // repeated
        } else if s.eq_ignore_ascii_case("Jun") { Ok(June)
        } else if s.eq_ignore_ascii_case("Jul") { Ok(July)
        } else if s.eq_ignore_ascii_case("Aug") { Ok(August)
        } else if s.eq_ignore_ascii_case("Sep") { Ok(September)
        } else if s.eq_ignore_ascii_case("Oct") { Ok(October)
        } else if s.eq_ignore_ascii_case("Nov") { Ok(November)
        } else if s.eq_ignore_ascii_case("Dec") { Ok(December)
        // abbr2
        } else if s.eq_ignore_ascii_case("JA") { Ok(January)
        } else if s.eq_ignore_ascii_case("FE") { Ok(February)
        } else if s.eq_ignore_ascii_case("MR") { Ok(March)
        } else if s.eq_ignore_ascii_case("AP") { Ok(April)
        } else if s.eq_ignore_ascii_case("MY") { Ok(May)
        } else if s.eq_ignore_ascii_case("JN") { Ok(June)
        } else if s.eq_ignore_ascii_case("JL") { Ok(July)
        } else if s.eq_ignore_ascii_case("AU") { Ok(August)
        } else if s.eq_ignore_ascii_case("SE") { Ok(September)
        } else if s.eq_ignore_ascii_case("OC") { Ok(October)
        } else if s.eq_ignore_ascii_case("NV") { Ok(November)
        } else if s.eq_ignore_ascii_case("DE") { Ok(December)
        // abbr1
        } else if s.eq_ignore_ascii_case("J") { Ok(January)
        } else if s.eq_ignore_ascii_case("F") { Ok(February)
        } else if s.eq_ignore_ascii_case("R") { Ok(March)
        } else if s.eq_ignore_ascii_case("P") { Ok(April)
        } else if s.eq_ignore_ascii_case("Y") { Ok(May)
        } else if s.eq_ignore_ascii_case("N") { Ok(June)
        } else if s.eq_ignore_ascii_case("L") { Ok(July)
        } else if s.eq_ignore_ascii_case("U") { Ok(August)
        } else if s.eq_ignore_ascii_case("S") { Ok(September)
        } else if s.eq_ignore_ascii_case("O") { Ok(October)
        } else if s.eq_ignore_ascii_case("V") { Ok(November)
        } else if s.eq_ignore_ascii_case("D") { Ok(December)
        //
        } else {
            Err("Invalid month name.")
        }
    }
}
