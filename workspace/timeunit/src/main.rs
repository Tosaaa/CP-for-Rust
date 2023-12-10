// Result handling + Custom enum type exercise

// with thiserror crate
// #[derive(Debug)]
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
#[derive(Error, Debug)]
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

fn main() {
    let res = match TimeUnit::new("sec") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
}

#[test]
fn timeunit_test() -> Result<(), TimeError> {
    let min = TimeUnit::new("minutes")?;
    assert_eq!(min, TimeUnit::Minutes);
    assert_eq!(min.plural(), "minutes");
    assert_eq!(min.singular(), "minute");
    Ok(())
}
