use chrono::Duration;
use solver_interface::SolverError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AocError {
    #[error("part not found")]
    PartNotFound,
    #[error("day {day} hasn't released yet. It releases {}:{:02}:{:02}:{:02} from now.",
	.duration.num_days(),
	.duration.num_hours() - .duration.num_days() * 24,
	.duration.num_minutes() - .duration.num_hours() * 60,
	.duration.num_seconds() - .duration.num_minutes() * 60)]
    HasNotReleasedYet { day: u32, duration: Duration },
    #[error("no test input found with the name {path}")]
    NoTestInputFound { path: Box<str> },
    #[error("io: {source}")]
    File {
        #[from]
        source: std::io::Error,
    },
    #[error("the file API_KEY was not found")]
    NoApiKey,
    #[error("I/O problem while reading API_KEY file: {source}")]
    ApiKeyIo { source: std::io::Error },
    #[error("request: {source}")]
    Request { source: Box<ureq::Error> },
    #[error("couldn't fetch prompt from network. Status {status}, content:\n{response}")]
    PromptResponse { status: u16, response: Box<str> },
    #[error("other: {source}")]
    OtherError {
        #[from]
        source: Box<dyn std::error::Error + Send + 'static>,
    },
    #[error("couldn't fetch input from network. Status: {status}\nContent:\n{response}")]
    InputResponse { status: u16, response: Box<str> },
    #[error("no day specified in argument `{arg}`")]
    NoDaySpecified { arg: Box<str> },
    #[error("could not parse `{part}` as integer in argument `{arg}`")]
    Parse { part: Box<str>, arg: Box<str> },
    #[error("non-UTF-8 data found in code block on the prompt page")]
    NonUtf8InPromptCodeBlock,
    #[error("non-UTF-8 data found in solution")]
    NonUtf8InSolution,
    #[error("fmt: {source}")]
    FmtError {
        #[from]
        source: std::fmt::Error,
    },
    #[error("day {0} not found")]
    DayNotFound(u32),
    #[error("argument was empty")]
    EmptyArgument,
    #[error("part was empty in {arg}")]
    EmptyPart { arg: Box<str> },
    #[error("too many test cases were generated from the prompt")]
    TooManyTestCases,
    #[error("answers did not match, exiting run")]
    IncorrectAnswer,
    #[error("{0} answers were incorrect.")]
    MultipleIncorrect(u32),
    #[error(transparent)]
    Solver {
        #[from]
        source: Box<SolverError>,
    },
    #[error("notify: {source}")]
    Watcher {
        #[from]
        source: Box<notify::Error>,
    },
}

macro_rules! from_error_boxed {
	($($from:ty, $var:ident;)*) => {
		$(
			impl From<$from> for AocError {
				fn from(value: $from) -> Self {
					Self::$var {
						source: Box::new(value),
					}
				}
			}
		)*
	};
}

from_error_boxed! {
    ureq::Error, Request;
    SolverError, Solver;
    notify::Error, Watcher;
}

impl AocError {
    pub fn no_test_input_found(s: impl Into<Box<str>>) -> Self {
        Self::NoTestInputFound { path: s.into() }
    }

    pub fn parse(part: impl Into<Box<str>>, arg: impl Into<Box<str>>) -> Self {
        Self::Parse {
            part: part.into(),
            arg: arg.into(),
        }
    }

    pub fn empty_part(arg: impl Into<Box<str>>) -> Self {
        Self::EmptyPart { arg: arg.into() }
    }

    pub fn prompt_response(status: u16, response: impl Into<Box<str>>) -> Self {
        Self::PromptResponse {
            status,
            response: response.into(),
        }
    }

    pub fn input_response(status: u16, response: impl Into<Box<str>>) -> Self {
        Self::InputResponse {
            status,
            response: response.into(),
        }
    }

    pub fn no_day_specified(arg: impl Into<Box<str>>) -> Self {
        Self::NoDaySpecified { arg: arg.into() }
    }
}
