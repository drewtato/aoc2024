mod error;
pub use error::AocError;
pub mod runner;

pub const YEAR: u32 = 2024;
pub type Res<T> = Result<T, AocError>;
pub type Settings = runner::Settings<YEAR>;
