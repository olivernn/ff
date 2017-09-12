#![feature(inclusive_range_syntax)]
extern crate termion;
extern crate ignore;

mod location;
mod jump;
pub mod index;
pub mod query;
pub mod ui;
mod min_set;
pub mod query_result;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
