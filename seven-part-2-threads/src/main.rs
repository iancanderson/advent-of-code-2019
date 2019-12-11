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

    for _ in vec![5,6,7,8,9] {
        // This channel goes out of the amplifier
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let amp = Amplifier { output: tx, input: last_input };

            let received = amp.input.recv().unwrap();
            amp.output.send(received + 1).unwrap();
        });

        last_input = rx;
    }

    // Send 0 into the first amplifier
    first_output.send(0).unwrap();
    let received = last_input.recv().unwrap();
    println!("Received: {}", received);
}

struct Amplifier {
    output: mpsc::Sender<i32>,
    input: mpsc::Receiver<i32>,
}
