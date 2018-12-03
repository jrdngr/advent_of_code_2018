pub mod challenges;
pub mod utils;

pub type AnyError = Box<dyn std::error::Error>;

fn main() -> Result<(), AnyError> {
    run_current();
    //run_previous();

    Ok(())
}

fn run_current() {
    //challenges::day3::day3_1();
    challenges::day3::day3_2();
}

fn run_previous() {
    challenges::day1::day1_1();
    challenges::day1::day1_2();
    challenges::day2::day2_1();
    challenges::day2::day2_2();
}
