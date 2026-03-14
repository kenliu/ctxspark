use std::env;
use std::process;

const BLOCKS: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
const VERSION: &str = env!("CARGO_PKG_VERSION");

struct Args {
    value: u8,
    low_color: u8,
    mid_color: u8,
    high_color: u8,
    mid_threshold: u8,
    high_threshold: u8,
    bg_color: u8,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            value: 0,
            low_color: 82,
            mid_color: 226,
            high_color: 196,
            mid_threshold: 50,
            high_threshold: 80,
            bg_color: 254,
        }
    }
}

fn print_help(bg: u8) {
    let g = colored_char(block_char(40), 82,  bg);
    let y = colored_char(block_char(65), 226, bg);
    let r = colored_char(block_char(90), 196, bg);
    let o = colored_char(block_char(85), 208, bg);
    print!(
        "ctxspark {VERSION}
Output a sparkline character for a context usage percentage (0-99).

The sparkline character is color-coded by usage level with a configurable
background color so the unfilled portion is visible. Output format: 65%[▅]

Usage: ctxspark [OPTIONS] <VALUE>

Arguments:
  <VALUE>              Context usage percentage (0-99)

Options:
      --low-color <COLOR>       ANSI 256 color for low usage (0-mid_threshold) [default: 82]
      --mid-color <COLOR>       ANSI 256 color for mid usage (mid_threshold-high_threshold) [default: 226]
      --high-color <COLOR>      ANSI 256 color for high usage (above high_threshold) [default: 196]
      --mid-threshold <PCT>     Percentage threshold between low and mid color [default: 50]
      --high-threshold <PCT>    Percentage threshold between mid and high color [default: 80]
      --bg-color <COLOR>        ANSI 256 background color [default: 254]
  -h, --help                    Print help
  -V, --version                 Print version

Examples:
  ctxspark 40                                  40%[{g}]  green (default)
  ctxspark 65                                  65%[{y}]  yellow (default)
  ctxspark 90                                  90%[{r}]  red (default)
  ctxspark 75 --mid-threshold 60 --high-threshold 90
  ctxspark 85 --high-color 208                 85%[{o}]  orange instead of red
"
    );
}

fn parse_args() -> Args {
    let mut args = Args::default();
    let mut argv = env::args().skip(1);
    let mut value_set = false;

    macro_rules! next_u8 {
        ($flag:expr) => {{
            let s = argv.next().unwrap_or_else(|| {
                eprintln!("error: {} requires a value", $flag);
                process::exit(1);
            });
            s.parse::<u8>().unwrap_or_else(|_| {
                eprintln!("error: {} value must be 0-255", $flag);
                process::exit(1);
            })
        }};
    }

    while let Some(arg) = argv.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help(args.bg_color);
                process::exit(0);
            }
            "-V" | "--version" => {
                println!("ctxspark {VERSION}");
                process::exit(0);
            }
            "--low-color"      => args.low_color      = next_u8!("--low-color"),
            "--mid-color"      => args.mid_color      = next_u8!("--mid-color"),
            "--high-color"     => args.high_color     = next_u8!("--high-color"),
            "--mid-threshold"  => args.mid_threshold  = next_u8!("--mid-threshold"),
            "--high-threshold" => args.high_threshold = next_u8!("--high-threshold"),
            "--bg-color"       => args.bg_color       = next_u8!("--bg-color"),
            other => {
                if other.starts_with('-') {
                    eprintln!("error: unknown option: {other}");
                    process::exit(1);
                }
                if value_set {
                    eprintln!("error: unexpected argument: {other}");
                    process::exit(1);
                }
                args.value = other.parse::<u8>().unwrap_or_else(|_| {
                    eprintln!("error: value must be an integer 0-99");
                    process::exit(1);
                });
                value_set = true;
            }
        }
    }

    if !value_set {
        eprintln!("error: missing required argument <VALUE>");
        eprintln!("Usage: ctxspark [OPTIONS] <VALUE>");
        eprintln!("  -h, --help   Print help");
        process::exit(1);
    }

    args
}

fn block_char(value: u8) -> char {
    let idx = (value as usize * 9) / 100;
    BLOCKS[idx.min(8)]
}

fn fg_color(value: u8, mid_threshold: u8, high_threshold: u8, low: u8, mid: u8, high: u8) -> u8 {
    if value <= mid_threshold {
        low
    } else if value <= high_threshold {
        mid
    } else {
        high
    }
}

fn colored_char(ch: char, fg: u8, bg: u8) -> String {
    format!("\x1b[38;5;{fg}m\x1b[48;5;{bg}m{ch}\x1b[0m")
}

fn main() {
    let args = parse_args();

    if args.value > 99 {
        eprintln!("Error: value must be 0-99");
        process::exit(1);
    }
    if args.mid_threshold >= args.high_threshold {
        eprintln!("Error: --mid-threshold must be less than --high-threshold");
        process::exit(1);
    }

    let ch = block_char(args.value);
    let fg = fg_color(
        args.value,
        args.mid_threshold,
        args.high_threshold,
        args.low_color,
        args.mid_color,
        args.high_color,
    );

    let colored = colored_char(ch, fg, args.bg_color);
    print!("{}%[{colored}]", args.value);
}

#[cfg(test)]
mod tests {
    use super::*;

    // block_char: transition points derived from (value * 9) / 100
    // 0-11→' ', 12-22→'▁', 23-33→'▂', 34-44→'▃', 45-55→'▄',
    // 56-66→'▅', 67-77→'▆', 78-88→'▇', 89-99→'█'

    #[test]
    fn block_char_boundaries() {
        assert_eq!(block_char(0), ' ');
        assert_eq!(block_char(11), ' ');
        assert_eq!(block_char(12), '▁');
        assert_eq!(block_char(22), '▁');
        assert_eq!(block_char(23), '▂');
        assert_eq!(block_char(44), '▃');
        assert_eq!(block_char(45), '▄');
        assert_eq!(block_char(55), '▄');
        assert_eq!(block_char(56), '▅');
        assert_eq!(block_char(66), '▅');
        assert_eq!(block_char(67), '▆');
        assert_eq!(block_char(77), '▆');
        assert_eq!(block_char(78), '▇');
        assert_eq!(block_char(88), '▇');
        assert_eq!(block_char(89), '█');
        assert_eq!(block_char(99), '█');
    }

    #[test]
    fn fg_color_default_thresholds() {
        // at and below mid_threshold (50) → low (82)
        assert_eq!(fg_color(0, 50, 80, 82, 226, 196), 82);
        assert_eq!(fg_color(50, 50, 80, 82, 226, 196), 82);
        // above mid, at and below high_threshold (80) → mid (226)
        assert_eq!(fg_color(51, 50, 80, 82, 226, 196), 226);
        assert_eq!(fg_color(80, 50, 80, 82, 226, 196), 226);
        // above high_threshold → high (196)
        assert_eq!(fg_color(81, 50, 80, 82, 226, 196), 196);
        assert_eq!(fg_color(99, 50, 80, 82, 226, 196), 196);
    }

    #[test]
    fn fg_color_custom_thresholds() {
        // mid=30, high=60
        assert_eq!(fg_color(30, 30, 60, 1, 2, 3), 1);
        assert_eq!(fg_color(31, 30, 60, 1, 2, 3), 2);
        assert_eq!(fg_color(60, 30, 60, 1, 2, 3), 2);
        assert_eq!(fg_color(61, 30, 60, 1, 2, 3), 3);
    }
}
