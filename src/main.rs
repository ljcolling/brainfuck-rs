fn main() {

    //
    // TODO:
    // 1. Read in filename;
    // 2. Load contents of file;
    // 3. Run code;
    // 4. Accept input;
    //

    // let input = "++>++[<+>-]++++++++[<++++++>-]<.";
    let input = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    let input: Vec<String> = input.chars().map(|x| x.to_string()).collect();
    bf(input);
}

fn bf(input: Vec<String>) {
    let mut output = String::new();
    let mut loc = 0usize;

    let mut mem = [0u8; 30_000];

    let mut skip = false;
    let mut anti = false;
    let mut paren: Vec<String> = vec![];
    let mut i = 0usize;
    let mut start = true;
    while i < input.len() - 1 {
        if start == true {
            start = false;
        } else {
            i += 1;
        }

        if skip == false && anti == false {
            let cmd = &input[i];
            if cmd == ">" {
                loc += 1;
            };

            if cmd == "<" {
                loc -= 1;
            };

            if cmd == "+" {
                mem[loc] += 1;
            };

            if cmd == "-" {
                mem[loc] -= 1;
            };

            if cmd == "." {
                // print!("{}", mem[loc] as char);
                output.push(mem[loc] as char);
            }

            if cmd == "," {
                unimplemented!();
            }

            if cmd == "[" {
                if mem[loc] == 0 {
                    skip = true;
                    paren.push("[".to_string());
                }
            }

            if cmd == "]" {
                if mem[loc] != 0 {
                    anti = true;
                    paren.push("]".to_string());
                }
            }
        }

        while skip == true {
            i += 1;
            let cmd = &input[i];
            if cmd == "[" {
                paren.push("[".to_string());
            };

            if cmd == "]" {
                paren.pop();
            }

            if paren.len() == 0 {
                skip = false;
            };
        }

        while anti == true {
            i -= 1;
            let cmd = &input[i];
            if cmd == "]" {
                paren.push("]".to_string());
            };

            if cmd == "[" {
                paren.pop();
            }
            if paren.len() == 0 {
                anti = false
            };
        }
    }

    println!("{}", output);
}

/*
#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(-2, -4 ; "when both operands are negative")]
    #[test_case(2,  4  ; "when both operands are positive")]
    #[test_case(4,  2  ; "when operands are swapped")]
    fn multiplication_tests(x: i8, y: i8) {
        let actual = (x * y).abs();

        assert_eq!(8, actual)
    }
}

*/
