use brainfuck::interpreter::{handler::Handler, parser};

fn main() {
    println!("Hello, world!");
    let code = parser::from_string_to_instructions(
        "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.",
    );
    let mut handler = Handler::default();
    handler.execute(&code);
    println!();
}
