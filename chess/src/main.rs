const DARK_GREEN: &str = "\x1b[48;2;16;105;9m";
const LIGHT_GREEN: &str = "\x1b[48;2;140;238;133m";
const BLACK_FONT: &str = "\x1b[30m";
const WHITE_FONT: &str = "\x1b[97m";
const RESET: &str = "\x1b[0m";
fn main() {
    let width: usize = 4;
    let x: char = '\u{265A}';
    println!(
        "{1}{5}{0:^width$}{2}{4}{0:^width$}{3}",
        x,
        DARK_GREEN,
        LIGHT_GREEN,
        RESET,
        BLACK_FONT,
        WHITE_FONT,
    );
    println!("222222");
}
