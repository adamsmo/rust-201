#![feature(coroutines)]
#![feature(coroutine_trait)]
#![feature(stmt_expr_attributes)]

mod data_pipeline;
mod fibonacci;
mod state_machine;

pub use data_pipeline::{moving_average_coroutine, outlier_detector_coroutine};
pub use fibonacci::fibonacci_coroutine;
pub use state_machine::ai_controller_coroutine;
