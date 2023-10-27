use lazy_static::lazy_static;
use tokio_util::sync::CancellationToken;

lazy_static! {
  // This cancellation token will be activated when the program receives a shutdown signal. It will
  // trigger cleanup tasks in active Tokio threads.
  pub static ref THREAD_CANCELLATION_TOKEN: CancellationToken= CancellationToken::new( );
}

pub static SERVER_ERROR: &'static str= "Server error occurred";