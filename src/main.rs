pub mod challenges;
pub mod utils;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    run_current()?;
    //run_previous();

    Ok(())
}

fn run_current() -> Result<()> {
    challenges::day5::day5_1()?;
    challenges::day5::day5_2();

    Ok(())
}

fn run_previous() -> Result<()> {
    challenges::day1::day1_1();
    challenges::day1::day1_2();
    challenges::day2::day2_1();
    challenges::day2::day2_2();
    challenges::day3::day3_1();
    challenges::day3::day3_2();
    challenges::day4::day4_1();
    challenges::day4::day4_2();

    Ok(())
}
