use std::thread;
use std::sync::mpsc;
use itertools::{Itertools, Permutations};

fn main() {
    let program = vec![3,8,1001,8,10,8,105,1,0,0,21,30,51,72,81,94,175,256,337,418,99999,3,9,101,5,9,9,4,9,99,3,9,1001,9,3,9,1002,9,2,9,1001,9,2,9,1002,9,5,9,4,9,99,3,9,1002,9,4,9,101,4,9,9,102,5,9,9,101,3,9,9,4,9,99,3,9,1002,9,4,9,4,9,99,3,9,102,3,9,9,1001,9,4,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,99,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,99];

    println!("Max thruster signal: {}", max_thruster_signal_with_feedback(&program));
}

// Simple version of program
//
// Each amplifier needs to be able to receive input from previous amplifier,
// and produce output for the next amplifier.
//
// Proof of concept program:
//
// Initialize each amplifier
// Arrange amplifiers in a sequence
// When an amplifier receives an input value, it produces that value + 1 to its output
//
// In the main thread:
// Provide a starting value to the first amplifier
// Read from the output of the last amplifier to get the result

fn max_thruster_signal_with_feedback(program: &Vec<i32>) -> i32 {
    // Loop over all permutations of [0,1,2,3,4]
    // Find permutation with max thruster signal

    return vec![5,6,7,8,9].iter().permutations(5).map(|phase_setting_sequence| {
        let signal = thruster_signal_with_feedback(&phase_setting_sequence, program);
        println!("Sequence {:?} gives output signal: {}", phase_setting_sequence, signal);
        return signal;
    }).max().expect("There will definitely be a maximum");
}

fn thruster_signal_with_feedback(phase_setting_sequence: &Vec<&i32>, program: &Vec<i32>) -> i32 {
    // This channel goes into the first amplifier
    let (first_output, first_input) = mpsc::channel();
    let mut last_input: mpsc::Receiver<i32> = first_input;

    let mut child_threads = vec![];

    for _ in phase_setting_sequence {
        // This channel goes out of the amplifier
        let (tx, rx) = mpsc::channel();

        child_threads.push(thread::spawn(move || {
            let amp = Amplifier { output: tx, input: last_input };

            let received = amp.input.recv().unwrap();
            amp.output.send(received + 1).unwrap();

            let received2 = amp.input.recv().unwrap();
            amp.output.send(received2 + 1).unwrap();
        }));

        last_input = rx;
    }

    // Send 0 into the first amplifier
    first_output.send(0).unwrap();

    let received = last_input.recv().unwrap();
    println!("Received: {}", received);

    first_output.send(received).unwrap();

    for child_thread in child_threads {
        let _ = child_thread.join();
    }

    let received2 = last_input.recv().unwrap();
    println!("Received: {}", received2);

    return received2;

    // How to know when all threads halt?
    // I guess the thread should return when they get a 99 instruction
}

struct Amplifier {
    output: mpsc::Sender<i32>,
    input: mpsc::Receiver<i32>,
}
