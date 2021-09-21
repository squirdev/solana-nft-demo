mod state;
mod instructions;
mod entrypoint;
mod error;
mod processor;

#[cfg(test)]
mod tests {
    use crate::state::{Mint, Account};

    #[test]
    fn it_works() {
        println!("{}", std::mem::size_of::<Mint>());
    }
}
