fn build_bracket_map(program: &str) -> Vec<usize> {
    let bytes = program.as_bytes();
    let len = bytes.len();
    let mut map = vec![0usize; len];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..len {
        match bytes[i] {
            b'[' => {
                stack.push(i);
            }
            b']' => {
                let open = stack.pop().expect("Unmatched ]");
                map[open] = i;
                map[i] = open;
            }
            _ => {}
        }
    }
    if !stack.is_empty() {
        panic!("Unmatched [");
    }
    map
}

fn main() {
    let program = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    let mut tape = [0u8; 30000];
    let mut dp: usize = 0;
    let mut pc: usize = 0;
}
