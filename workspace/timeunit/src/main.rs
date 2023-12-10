// Result handling + Custom enum type exercise

// with thiserror crate
// #[derive(Debug, Clone)]
// pub struct TimeError {
//     pub message: String,
// }

// use std::fmt;
// impl fmt::Display for TimeError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
//         write!(f, "{}", self.message)
//     }
// }

// impl std::error::Error for TimeError {}


// with thiserror crate
use thiserror::Error;
#[derive(Error, Debug, Clone)]
#[error("{message}")]
pub struct TimeError {
    pub message: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TimeUnit {
    Seconds, Minutes, Hours, Days, Months, Years
}

impl TimeUnit {
    fn new(unit: &str) -> Result<TimeUnit, TimeError> {
        match unit {
            "seconds" => Ok(TimeUnit::Seconds),
            "minutes" => Ok(TimeUnit::Minutes),
            "hours" => Ok(TimeUnit::Hours),
            "days" => Ok(TimeUnit::Days),
            "months" => Ok(TimeUnit::Months),
            "years" => Ok(TimeUnit::Years),
            _ => Err(TimeError { message: "unit not found.".to_string() })
        }
    }

    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years"
        }
    }

    fn singular(self) -> &'static str {
        self.plural().trim_end_matches("s")
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

fn main() {
    let res = match TimeUnit::new("seconds") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let four_score_and_seven_years_ago = 
        RoughTime::InThePast(TimeUnit::Years, 4*20 + 7);
    let three_hours_from_now =
        RoughTime::InTheFuture(TimeUnit::Hours, 3);
}

#[test]
fn timeunit_test() -> Result<(), TimeError> {
    let min = TimeUnit::new("minutes")?;
    assert_eq!(min, TimeUnit::Minutes);
    assert_eq!(min.plural(), "minutes");
    assert_eq!(min.singular(), "minute");
    Ok(())
}
