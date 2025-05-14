use snafu::Snafu;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
  
  #[snafu(display("Error: {}", message))]
  PlainText {
    message: String,
  }
}