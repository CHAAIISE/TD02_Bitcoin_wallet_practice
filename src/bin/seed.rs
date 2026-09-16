use TD2::generate_seed

fn main() -> Result<(), getrandom::Error> {
    TD2::generate_seed()
}