#![feature(inclusive_range_syntax)]
extern crate termion;
extern crate ignore;

mod location;
mod jump;
pub mod index;
pub mod query;
mod min_set;
mod query_result;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
