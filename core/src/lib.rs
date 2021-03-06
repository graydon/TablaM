//pub mod btree;
pub mod dsl;
pub mod macros;
pub mod range;
pub mod relational;
pub mod scalars;
pub mod schema;
pub mod sequence;
pub mod stdlib;
pub mod table;
pub mod types;
pub mod vector;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
