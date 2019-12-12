use std::thread;
use std::sync::mpsc;

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

fn main() {
    // This channel goes into the first amplifier
    let (first_output, first_input) = mpsc::channel();
    let mut last_input: mpsc::Receiver<i32> = first_input;

    let mut child_threads = vec![];

    for _ in vec![5,6,7,8,9] {
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

    // How to know when all threads halt?
    // I guess the thread should return when they get a 99 instruction
}

struct Amplifier {
    output: mpsc::Sender<i32>,
    input: mpsc::Receiver<i32>,
}
