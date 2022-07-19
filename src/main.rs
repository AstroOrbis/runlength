fn main() {
    // get all args and concat it into a string
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    let concat: String = args.join(" ");


    

    let output: String = encode(&concat);

    println!("\n{}\n", output);
}

fn encode(input: &String) -> String {
    let mut output: String = String::new();
    let mut count: i32;
    let mut i: usize = 0;
    let mut currentchar: char;
    let len: usize = input.chars().count();
    let s: Vec<char> = input.chars().collect();

    while i < len {
        currentchar = s[i];
        count = 1;

        while (i + 1 < len) && currentchar == s[i + 1] {
            i += 1;
            count += 1;
        }

        if count == 1 {
            output = format!("{}{}", output, currentchar)
        } else {
            output = format!("{}{}{}", output, count, currentchar);
        }

        i += 1;
    

    }

    output
}
