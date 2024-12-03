// devela::sys::time::calendar::week
//
//!
//

use crate::{Display, FmtResult, Formatter, FromStr};
#[allow(clippy::enum_glob_use)]
use Weekday::*;

/// The days of the week.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Weekday {
    /// The first day of the week, according to the ISO-8601 standard.
    ///
    /// # Etymology
    /// The name Monday comes from the Old English word “Monandæg”,
    /// which means “Moon’s day”.
    Monday = 0,

    ///
    /// # Etymology
    /// The name Tuesday comes from the Old English word “Tiwesdæg”, which means
    /// “Tiw’s day”. Tiw was an Anglo-Saxon god associated with war and combat.
    Tuesday,

    ///
    /// # Etymology
    /// The name Wednesday comes from the Old English word “Wodnesdæg”, which
    /// means “Woden’s day”. Woden was the chief god in Norse mythology,
    /// associated with wisdom, war, and death.
    Wednesday,

    ///
    /// # Etymology
    /// The name Thursday comes from the Old English word “Þunresdæg”, which
    /// means “Thor’s day”. Thor was the Norse god of thunder and lightning.
    Thursday,

    ///
    /// # Etymology
    /// The name Friday comes from the Old English word “Frīgedæg”, which means
    /// “Frige’s day”. Frige was an Anglo-Saxon goddess associated with love,
    /// fertility, and domestic life.
    Friday,

    ///
    /// # Etymology
    /// The name Saturday comes from the Latin word “Saturni dies”, which means
    /// “Saturn’s day”. Saturn was the Roman god of agriculture and wealth.
    Saturday,

    ///
    /// # Etymology
    /// The name Sunday comes from the Old English word “Sunandæg”, which means
    /// “Sun’s day”.
    Sunday,
}

impl Weekday {
    /// The number of weekdays in a week.
    pub const COUNT: usize = 7;

    /// Returns the previous weekday.
    pub const fn previous(self) -> Weekday {
        self.previous_nth(1)
    }

    /// Returns the previous `nth` weekday.
    pub const fn previous_nth(self, nth: usize) -> Weekday {
        Self::from_monday_index_unchecked(self.index_from_monday().wrapping_sub(nth) % Self::COUNT)
    }

    /// Returns the next weekday,
    pub const fn next(self) -> Weekday {
        self.next_nth(1)
    }

    /// Returns the next `nth` weekday.
    pub const fn next_nth(self, nth: usize) -> Weekday {
        Self::from_monday_index_unchecked(self.index_from_monday().wrapping_add(nth) % Self::COUNT)
    }
}

/// # from Monday
impl Weekday {
    /* to number */

    /// Returns the weekday number from `Monday=1` to `Sunday=7`.
    pub const fn number_from_monday(self) -> u8 {
        self.index_from_monday() as u8 + 1
    }

    /// Returns the weekday index from `Monday=0` to `Sunday=6`.
    pub const fn index_from_monday(self) -> usize {
        self as _
    }

    /// Returns a weekday from its counting number, from `Monday=1` to `Sunday=7`.
    ///
    /// # Errors
    /// `if n < 1 || n > 7`
    pub const fn from_monday_number(n: u8) -> Result<Weekday, &'static str> {
        match n {
            1 => Ok(Monday),
            2 => Ok(Tuesday),
            3 => Ok(Wednesday),
            4 => Ok(Thursday),
            5 => Ok(Friday),
            6 => Ok(Saturday),
            7 => Ok(Sunday),
            _ => Err("The weekday number must be between 1 and 7."),
        }
    }

    /// Returns a weekday from its index, from `Monday=0` to `Sunday=6`.
    ///
    /// # Errors
    /// `if index > 6`
    pub const fn from_monday_index(index: usize) -> Result<Weekday, &'static str> {
        match index {
            0 => Ok(Monday),
            1 => Ok(Tuesday),
            2 => Ok(Wednesday),
            3 => Ok(Thursday),
            4 => Ok(Friday),
            5 => Ok(Saturday),
            6 => Ok(Sunday),
            _ => Err("The weekday number must be between 0 and 6."),
        }
    }

    /// Returns a weekday from its index, from `Monday=0` to `Sunday=6`.
    ///
    /// # Panics
    /// `if index > 6`
    pub const fn from_monday_index_unchecked(index: usize) -> Self {
        match index {
            0 => Monday,
            1 => Tuesday,
            2 => Wednesday,
            3 => Thursday,
            4 => Friday,
            5 => Saturday,
            6 => Sunday,
            _ => panic!("The weekday number must be between 0 and 6."),
        }
    }
}

/// # from Sunday
impl Weekday {
    /// Returns the weekday number from `Sunday=1` to `Monday=7`.
    pub const fn number_from_sunday(self) -> u8 {
        self.index_from_sunday() as u8 + 1
    }

    /// Returns the weekday index from `Sunday=0` to `Monday=6`.
    pub const fn index_from_sunday(self) -> usize {
        match self {
            Monday => 1,
            Tuesday => 2,
            Wednesday => 3,
            Thursday => 4,
            Friday => 5,
            Saturday => 6,
            Sunday => 0,
        }
    }

    /// Returns a weekday from its counting number, from `Sunday=1` to `Monday=7`.
    ///
    /// # Errors
    /// `if n < 1 || n > 7`
    pub const fn from_sunday_number(n: u8) -> Result<Weekday, &'static str> {
        match n {
            1 => Ok(Sunday),
            2 => Ok(Monday),
            3 => Ok(Tuesday),
            4 => Ok(Wednesday),
            5 => Ok(Thursday),
            6 => Ok(Friday),
            7 => Ok(Saturday),
            _ => Err("The weekday number must be between 1 and 7."),
        }
    }

    /// Returns a weekday from its index, from `Sunday=0` to `Monday=6`.
    ///
    /// # Errors
    /// `if index > 6`
    pub const fn from_sunday_index(index: usize) -> Result<Weekday, &'static str> {
        match index {
            0 => Ok(Sunday),
            1 => Ok(Monday),
            2 => Ok(Tuesday),
            3 => Ok(Wednesday),
            4 => Ok(Thursday),
            5 => Ok(Friday),
            6 => Ok(Saturday),
            _ => Err("The weekday number must be between 0 and 6."),
        }
    }

    /// Returns a weekday from its index, from `Sunday=0` to `Monday=6`.
    ///
    /// # Panics
    /// `if index > 6`
    pub const fn from_sunday_index_unchecked(index: usize) -> Self {
        match index {
            0 => Sunday,
            1 => Monday,
            2 => Tuesday,
            3 => Wednesday,
            4 => Thursday,
            5 => Friday,
            6 => Saturday,
            _ => panic!("The weekday number must be between 0 and 6."),
        }
    }
}

/// # abbreviations & representations
#[allow(missing_docs, non_upper_case_globals)]
impl Weekday {
    /// Returns the 3-letter abbreviated weekday name, in ASCII, UpperCamelCase.
    pub fn abbr3(self) -> &'static str {
        match self {
            Monday => "Mon",
            Tuesday => "Tue",
            Wednesday => "Wed",
            Thursday => "Thu",
            Friday => "Fri",
            Saturday => "Sat",
            Sunday => "Sun",
        }
    }

    pub const Mon: Weekday = Weekday::Monday;
    pub const Tue: Weekday = Weekday::Tuesday;
    pub const Wed: Weekday = Weekday::Wednesday;
    pub const Thu: Weekday = Weekday::Thursday;
    pub const Fri: Weekday = Weekday::Friday;
    pub const Sat: Weekday = Weekday::Saturday;
    pub const Sun: Weekday = Weekday::Sunday;

    /// Returns the 2-letter abbreviated weekday name, in ASCII, UPPERCASE.
    pub fn abbr2(self) -> &'static str {
        match self {
            Monday => "MO",
            Tuesday => "TU",
            Wednesday => "WE",
            Thursday => "TH",
            Friday => "FR",
            Saturday => "SA",
            Sunday => "SU",
        }
    }

    pub const MO: Weekday = Weekday::Monday;
    pub const TU: Weekday = Weekday::Tuesday;
    pub const WE: Weekday = Weekday::Wednesday;
    pub const TH: Weekday = Weekday::Thursday;
    pub const FR: Weekday = Weekday::Friday;
    pub const SA: Weekday = Weekday::Saturday;
    pub const SU: Weekday = Weekday::Sunday;

    /// Returns the 1-letter abbreviated weekday name, in ASCII, UPPERCASE.
    pub fn abbr1(self) -> &'static str {
        match self {
            Monday => "M",
            Tuesday => "T",
            Wednesday => "W",
            Thursday => "H",
            Friday => "F",
            Saturday => "A",
            Sunday => "U",
        }
    }

    pub const M: Weekday = Weekday::Monday;
    pub const T: Weekday = Weekday::Tuesday;
    pub const W: Weekday = Weekday::Wednesday;
    pub const H: Weekday = Weekday::Thursday;
    pub const F: Weekday = Weekday::Friday;
    pub const A: Weekday = Weekday::Saturday;
    pub const U: Weekday = Weekday::Sunday;

    /// Returns the emoji associated to the weekday.
    ///
    /// These are: 🌕, 🏹, 🧙, ⚡, 💕, 💰, 🌞.
    ///
    /// Full Moon, Bow and Arrow, Mage, Lightning Bolt, Two Hearts, Money Bag,
    /// and Sun.
    ///
    /// # Examples
    /// ```
    /// # use devela::Weekday;
    /// assert_eq![Weekday::Thursday.emoji(), '⚡'];
    /// ```
    pub const fn emoji(self) -> char {
        match self {
            // Full Moon,
            Monday => '🌕',
            // Bow and Arrow.
            Tuesday => '🏹',
            // Mage.
            Wednesday => '🧙',
            // Lightning Bolt.
            Thursday => '⚡',
            // Two Hearts.
            Friday => '💕',
            // .
            Saturday => '💰',
            // Sun
            Sunday => '🌞',
        }
    }

    /// Returns the char of the associated planet of Helenistic astrology.
    ///
    /// These are: ☽, ♂, ☿, ♃, ♀, ♄, ☀.
    ///
    /// # Examples
    /// ```
    /// # use devela::Weekday;
    /// assert_eq![Weekday::Thursday.planet(), '♃'];
    /// ```
    pub const fn planet(self) -> char {
        match self {
            // Moon.
            Monday => '☽',
            // Mars.
            Tuesday => '♂',
            // Mercury.
            Wednesday => '☿',
            // Jupiter.
            Thursday => '♃',
            // Venus.
            Friday => '♀',
            // Saturn.
            Saturday => '♄',
            // Sun.
            Sunday => '☀',
        }
    }

    /// Returns the name of the associated planet of Helenistic astrology.
    ///
    /// These are: Moon, Mars, Mercury, Jupiter, Venus, Saturn and Sun.
    ///
    /// # Examples
    /// ```
    /// # use devela::Weekday;
    /// assert_eq![Weekday::Thursday.planet_name(), "Jupiter"];
    /// ```
    pub const fn planet_name(self) -> &'static str {
        match self {
            Monday => "Moon",
            Tuesday => "Mars",
            Wednesday => "Mercury",
            Thursday => "Jupiter",
            Friday => "Venus",
            Saturday => "Saturn",
            Sunday => "Sun",
        }
    }
}

impl Display for Weekday {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        f.write_str(match self {
            Monday => "Monday",
            Tuesday => "Tuesday",
            Wednesday => "Wednesday",
            Thursday => "Thursday",
            Friday => "Friday",
            Saturday => "Saturday",
            Sunday => "Sunday",
        })
    }
}

/// Returns a `Weekday` from a string containing either the full weekday name,
/// or any of the weekday ASCII abbreviations.
impl FromStr for Weekday {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Weekday, Self::Err> {
        // full name
        if s.eq_ignore_ascii_case("Monday") {
            Ok(Monday)
        } else if s.eq_ignore_ascii_case("Tuesday") {
            Ok(Tuesday)
        } else if s.eq_ignore_ascii_case("Wednesday") {
            Ok(Wednesday)
        } else if s.eq_ignore_ascii_case("Thursday") {
            Ok(Thursday)
        } else if s.eq_ignore_ascii_case("Friday") {
            Ok(Friday)
        } else if s.eq_ignore_ascii_case("Saturday") {
            Ok(Saturday)
        } else if s.eq_ignore_ascii_case("Sunday") {
            Ok(Sunday)
        // abbr3
        } else if s.eq_ignore_ascii_case("Mon") {
            Ok(Monday)
        } else if s.eq_ignore_ascii_case("Tue") {
            Ok(Tuesday)
        } else if s.eq_ignore_ascii_case("Wed") {
            Ok(Wednesday)
        } else if s.eq_ignore_ascii_case("Thu") {
            Ok(Thursday)
        } else if s.eq_ignore_ascii_case("Fri") {
            Ok(Friday)
        } else if s.eq_ignore_ascii_case("Sat") {
            Ok(Saturday)
        } else if s.eq_ignore_ascii_case("Sun") {
            Ok(Sunday)
        // abbr2
        } else if s.eq_ignore_ascii_case("MO") {
            Ok(Monday)
        } else if s.eq_ignore_ascii_case("TU") {
            Ok(Tuesday)
        } else if s.eq_ignore_ascii_case("WE") {
            Ok(Wednesday)
        } else if s.eq_ignore_ascii_case("TH") {
            Ok(Thursday)
        } else if s.eq_ignore_ascii_case("FR") {
            Ok(Friday)
        } else if s.eq_ignore_ascii_case("SA") {
            Ok(Saturday)
        } else if s.eq_ignore_ascii_case("SU") {
            Ok(Sunday)
        // abbr1
        } else if s.eq_ignore_ascii_case("M") {
            Ok(Monday)
        } else if s.eq_ignore_ascii_case("T") {
            Ok(Tuesday)
        } else if s.eq_ignore_ascii_case("W") {
            Ok(Wednesday)
        } else if s.eq_ignore_ascii_case("H") {
            Ok(Thursday)
        } else if s.eq_ignore_ascii_case("F") {
            Ok(Friday)
        } else if s.eq_ignore_ascii_case("S") {
            Ok(Saturday)
        } else if s.eq_ignore_ascii_case("U") {
            Ok(Sunday)

        //
        } else {
            Err("Invalid weekday name.")
        }
    }
}
