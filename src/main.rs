// See oxidecomputer/omicron#1146.  This program looks at memory dumps (created
// in the way described by that issue) and looks for runs of addresses that look
// like they could have been victims of illumos bug 15254.

#[derive(Debug)]
struct RunStart {
    addr: String,
    p1: String,
    p2: String,
}

#[derive(Debug)]
enum ParseState {
    None,
    ExpectZero(usize, RunStart),
    ExpectSame(usize, RunStart),
}

const P_ZERO: &'static str = "0000000000000000";
const MIN_RUN_LENGTH: usize = 12;

fn main() {
    let mut state = ParseState::None;

    for line in std::io::stdin().lines() {
        let line = line.expect("failed to read line");
        if line.len() == 0 {
            continue;
        }

        let (addr, p1, p2) = split_line(&line);
        let is_zero = p1 == P_ZERO && p2 == P_ZERO;
        eprintln!("line: {} ({})", line, is_zero);
        eprintln!("before state: {:?}", state);
        state = match state {
            ParseState::None if is_zero => ParseState::None,
            ParseState::None => ParseState::ExpectZero(
                0,
                RunStart {
                    addr: addr.to_string(),
                    p1: p1.to_string(),
                    p2: p2.to_string(),
                },
            ),
            ParseState::ExpectZero(c, r) if is_zero => ParseState::ExpectSame(c + 1, r),
            ParseState::ExpectSame(c, r) if r.p1 == p1 && r.p2 == p2 => {
                ParseState::ExpectZero(c, r)
            }

            ParseState::ExpectZero(c, r) => {
                if c >= MIN_RUN_LENGTH {
                    println!("found run of {} at addr {}", c, r.addr);
                }
                if is_zero {
                    ParseState::None
                } else {
                    ParseState::ExpectZero(
                        0,
                        RunStart {
                            addr: addr.to_string(),
                            p1: p1.to_string(),
                            p2: p2.to_string(),
                        },
                    )
                }
            }
            ParseState::ExpectSame(c, r) => {
                if c >= MIN_RUN_LENGTH {
                    println!("found run of {} at addr {}", c, r.addr);
                }
                if is_zero {
                    ParseState::None
                } else {
                    ParseState::ExpectZero(
                        0,
                        RunStart {
                            addr: addr.to_string(),
                            p1: p1.to_string(),
                            p2: p2.to_string(),
                        },
                    )
                }
            }
        };
        eprintln!("new state: {:?}", state);
    }
}

fn split_line(line: &str) -> (&str, &str, &str) {
    let parts = line.split_whitespace().collect::<Vec<_>>();
    assert_eq!(parts.len(), 3);
    (parts[0], parts[1], parts[2])
}
