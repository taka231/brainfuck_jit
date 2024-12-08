use brainfuck_jit::{compiler::Compiler, interpreter::Runtime, parser};

fn main() {
    let start = std::time::Instant::now();
    // unsafe {
    //     let mut compiler = Compiler::new(include_str!("../tests/factor.bf"));
    //     compiler.compile();
    //     compiler.run();
    // }
    let mut runtime = Runtime::new(include_str!("../tests/factor.bf"));
    runtime.run();
    let end = start.elapsed();
    println!("time: {}ms", end.as_millis());
}
