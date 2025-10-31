pub trait StringExt {
    fn find_next_char_index(&self, start_index: usize, target: char) -> usize;
    fn find_next_delimiter_index(&self, start_index: usize, delimiter_pair: (char, char)) -> usize;
}

impl StringExt for String {
    fn find_next_char_index(&self, start_index: usize, target: char) -> usize {
        let mut chars = self[start_index..self.len()].chars();
        let mut distance = 0;
        while let Some(value) = &chars.next() {
            if value != &target {
                distance += 1;
            } else {
                break;
            }
        }
        start_index + distance
    }

    fn find_next_delimiter_index(&self, start_index: usize, delimiter_pair: (char, char)) -> usize {
        let mut chars = self[start_index..self.len()].chars();
        let mut delimiters = 0;
        let mut distance = 0;

        while let Some(value) = &chars.next() {
            if value == &delimiter_pair.0 {
                delimiters += 1;
            }
            if value == &delimiter_pair.1 {
                delimiters -= 1;
            }
            if delimiters == 0 {
                break;
            }
            distance += 1;
        }
        start_index + distance
    }
}
