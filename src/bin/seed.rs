use td2::generate_seed

fn main() -> Result<Vec<String>, getrandom::Error> {
    td2::generate_seed()
}