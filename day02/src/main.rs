use intcode::Runner;

fn main() {
    let program = util::read_intcode();

    {
        let mut program = program.clone();
        program[1] = 12;
        program[2] = 2;
        let mut runner = Runner::new(&mut program);
        runner.run();
        println!("part 1: {}", program[0]);
    }

    {
        for noun in 0..100 {
            for verb in 0..100 {
                let mut program = program.clone();
                program[1] = noun;
                program[2] = verb;
                let mut runner = Runner::new(&mut program);
                runner.run();
                if program[0] == 19_690_720 {
                    println!("part 2: {}", 100 * noun + verb);
                    break;
                }
            }
        }
    }
}
