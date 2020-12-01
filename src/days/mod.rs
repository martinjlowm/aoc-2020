mod december_01;

pub fn solve(day: i32) {
    match day {
        1 => december_01::solve(),
        _ => print!("{}", "noop"),
    }
}
