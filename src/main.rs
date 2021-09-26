use std::cmp;

fn main() {
    for i in (1..255).step_by(10) {
        for j in (1..255).step_by(10) {
            print!("\x1b[38;2;{};{};{}mWW\x1b[0m ", cmp::max(0, 250 - i - j), i, j);
        }
        println!();
    }
}
