use hyper;
use serde_json;
use std::{error, fmt, io, result};

pub(crate) type Result<T> = result::Result<T, CloudwatchNotifierError>;

#[derive(Debug)]
pub(crate) enum CloudwatchNotifierError {
  Io(io::Error),
  Json(serde_json::Error),
  AutoSubscriptionError(hyper::error::Error),
  AutoSubscriptionBadStatus(String, hyper::status::StatusCode),
}

impl fmt::Display for CloudwatchNotifierError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      CloudwatchNotifierError::Io(ref e) => write!(f, "IO error: {}", e),
      CloudwatchNotifierError::Json(ref e) => write!(f, "JSON error: {}", e),
      CloudwatchNotifierError::AutoSubscriptionError(ref e) => {
        write!(f, "Failed to auto-subscribe: {}", e)
      }
      CloudwatchNotifierError::AutoSubscriptionBadStatus(ref url, ref code) => {
        write!(f, "Failed to auto-subscribe using {}, got status code {} ", url, code)
      }
    }
  }
}

impl error::Error for CloudwatchNotifierError {
  fn description(&self) -> &str {
    match *self {
      CloudwatchNotifierError::Io(ref e) => e.description(),
      CloudwatchNotifierError::Json(ref e) => e.description(),
      CloudwatchNotifierError::AutoSubscriptionError(ref e) => e.description(),
      CloudwatchNotifierError::AutoSubscriptionBadStatus(..) => "Auto-subscribe error",
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      CloudwatchNotifierError::Io(ref e) => Some(e),
      CloudwatchNotifierError::Json(ref e) => Some(e),
      CloudwatchNotifierError::AutoSubscriptionError(ref e) => Some(e),
      CloudwatchNotifierError::AutoSubscriptionBadStatus(..) => None,
    }
  }
}

impl From<serde_json::Error> for CloudwatchNotifierError {
  fn from(error: serde_json::Error) -> Self {
    CloudwatchNotifierError::Json(error)
  }
}

impl From<io::Error> for CloudwatchNotifierError {
  fn from(error: io::Error) -> Self {
    CloudwatchNotifierError::Io(error)
  }
}

impl From<hyper::error::Error> for CloudwatchNotifierError {
  fn from(error: hyper::error::Error) -> Self {
    CloudwatchNotifierError::AutoSubscriptionError(error)
  }
}
