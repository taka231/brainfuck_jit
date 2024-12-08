use brainfuck_jit::{compiler::Compiler, interpreter::Runtime};

fn main() {
    let mut runtime = Runtime::new(include_str!("../tests/factor.bf"));
    runtime.run();
    // unsafe {
    //     let mut compiler = Compiler::new(include_str!("../tests/factor.bf"));
    //     compiler.compile();
    //     compiler.run();
    // }
}
