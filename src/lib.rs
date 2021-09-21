mod state;

#[cfg(test)]
mod tests {
    use crate::state::{Mint, Account};

    #[test]
    fn it_works() {
        println!("{}", std::mem::size_of::<Mint>());
    }
}
