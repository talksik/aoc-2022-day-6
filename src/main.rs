#[derive(Debug)]
struct Message(Vec<char>);

impl Message {
    fn add_char(&mut self, c: char) {
        self.0.push(c);
    }

    fn add_chars(&mut self, chars: Vec<char>) {
        for c in chars {
            self.add_char(c);
        }
    }

    fn get_chars(&self) -> Vec<char> {
        self.0.clone()
    }

    fn get_count(&self) -> usize {
        self.0.len()
    }

    // check that each char is unique, and that there are 4 characters
    fn is_winning(&self) -> bool {
        let mut unique_chars: Vec<char> = Vec::new();

        for &c in self.0.iter() {
            if !unique_chars.contains(&c) {
                unique_chars.push(c);
            }
        }

        unique_chars.len() == 4
    }

    fn is_winning_two(&self) -> bool {
        let mut unique_chars: Vec<char> = Vec::new();

        for &c in self.0.iter() {
            if !unique_chars.contains(&c) {
                unique_chars.push(c);
            }
        }

        unique_chars.len() == self.get_count()
    }
}

// part 2
fn get_first_message_marker(sequence: &str) -> u32 {
    for (i, c) in sequence.chars().enumerate() {
        let mut message = Message(Vec::new());
        message.add_char(c);

        // get the next 3 characters
        for j in i + 1..i + 14 {
            // account for end of entire sequence
            if j < sequence.len() {
                message.add_char(
                    sequence
                        .chars()
                        .nth(j)
                        .expect("unable to get char at position"),
                );
            }
        }

        // check if we have a winning sequence and return the index after
        if message.is_winning_two() {
            return (i + 14) as u32;
        }
    }

    0
}

// part 1
// gets the index after the first marker sequence
fn get_first_marker_index(sequence: &str) -> u32 {
    // go through and start a new sequence every character
    // if we have a repeat for any of the sequences, delete that sequence
    // return if we have a sequence with 4 unique characters and return the index after

    for (i, c) in sequence.chars().enumerate() {
        let mut message = Message(Vec::new());
        message.add_char(c);

        // get the next 3 characters
        for j in i + 1..i + 4 {
            // account for end of entire sequence
            if j < sequence.len() {
                message.add_char(
                    sequence
                        .chars()
                        .nth(j)
                        .expect("unable to get char at position"),
                );
            }
        }

        // check if we have a winning sequence and return the index after
        if message.is_winning() {
            return (i + 4) as u32;
        }
    }

    0
}

fn read_input() -> String {
    include_str!("../input.txt").to_string()
}

fn main() {
    println!("Hello, world!");

    let input = read_input();
    let answer = get_first_marker_index(&input);

    println!("part 1: {}", answer);

    let answer_two = get_first_message_marker(&input);
    println!("part 2: {}", answer_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    // part 1
    #[test]
    fn test_simple_sequences_marker() {
        let sequence_one = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let sequence_two = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let sequence_three = "nppdvjthqldpwncqszvftbrmjlhg";
        let sequence_four = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let sequence_five = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        assert_eq!(get_first_marker_index(sequence_one), 7);
        assert_eq!(get_first_marker_index(sequence_two), 5);
        assert_eq!(get_first_marker_index(sequence_three), 6);
        assert_eq!(get_first_marker_index(sequence_four), 10);
        assert_eq!(get_first_marker_index(sequence_five), 11);
    }

    #[test]
    fn test_part_two() {
        let sequence_one = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let sequence_two = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let sequence_three = "nppdvjthqldpwncqszvftbrmjlhg";
        let sequence_four = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let sequence_five = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        assert_eq!(get_first_message_marker(sequence_one), 19);
        assert_eq!(get_first_message_marker(sequence_two), 23);
        assert_eq!(get_first_message_marker(sequence_three), 23);
        assert_eq!(get_first_message_marker(sequence_four), 29);
        assert_eq!(get_first_message_marker(sequence_five), 26);
    }
}
