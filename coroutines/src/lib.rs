#![feature(coroutines)]
#![feature(coroutine_trait)]
#![feature(stmt_expr_attributes)]

mod fibonacci;

pub use fibonacci::fibonacci_coroutine;
