use location::{Location, Locations, LocationLabel};

#[derive(Debug)]
pub struct Jump {
    pub source: Location,
    pub destination: Location,
    pub score: usize
}

pub struct Jumps {
    locations: Vec<Location>,
    source_index: usize,
    destination_index: usize,
    upper_jumps: Vec<Jump>
}

impl Jumps {
    pub fn new(path: &str) -> Self {
        let locations = Locations::new(path).collect();

        Jumps {
            locations: locations,
            source_index: 0,
            destination_index: 1,
            upper_jumps: Vec::new()
        }
    }
}

impl Iterator for Jumps {
    type Item = Jump;

    fn next(&mut self) -> Option<Jump> {
        if self.upper_jumps.len() > 0 {
            return self.upper_jumps.pop();
        }

        if self.destination_index >= self.locations.len() {                                                                                                             
            self.source_index = self.source_index + 1;                                                                                                        
            self.destination_index  = self.source_index + 1;                                                                                                       
        }                                                                                                                                                     
                                                                                                                                                              
        if self.destination_index >= self.locations.len() {                                                                                                             
            return None;                                                                                                                                      
        }                                                                                                                                                     

        let source = &self.locations[self.source_index];
        let destination = &self.locations[self.destination_index];

        let score = calculate_score(&source, &destination);

        if destination.character.is_uppercase() {
            for lower_destination_character in destination.character.to_lowercase() {
                let mut lower_destination = destination.clone();
                lower_destination.character = lower_destination_character;

                self.upper_jumps.push(
                    Jump {
                        source: source.clone(),
                        destination: lower_destination,
                        score: score - 1
                    }
                )
            }
        }

        let jump = Jump {
            source: source.clone(),
            destination: destination.clone(),
            score: score
        };
                                                                                                                                                              
        self.destination_index = self.destination_index + 1;                                                                                                            
                                                                                                                                                              
        Some(jump)                                      
    }
}

fn calculate_score(source: &Location, destination: &Location) -> usize {
    let score = destination.height;

    match (source.label, destination.label) {
        // its free to get to the end of the path
        (_, LocationLabel::PathEnd) => {
            0
        },

        // its cheap to go from word begin to word begin
        (LocationLabel::WordBegin, LocationLabel::WordBegin) => {
            score + 2
        },

        // ending on a word begin is only slightly more expensive
        (_, LocationLabel::WordBegin) => {
            score + 3
        },

        (LocationLabel::PathBegin, _) => {
            score + 5
        }

        _ => {
            // if the locations are contiguous then its _very_ cheap
            if destination.index == source.index + 1 {
                score + 1
            
            // otherwise its expensive
            } else {
                score + 5
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jump_count() {
        let jumps = Jumps::new("ab");
        // ^ => a | ^ => b | ^ => $
        // a => b | a => $
        // b => $
        assert_eq!(6, jumps.count());
    }

    #[test]
    fn jump_source_characters() {
        // ^ => a | ^ => b | ^ => $
        // a => b | a => $
        // b => $

        let actual: Vec<char> = Jumps::new("ab")
            .map(|jump| jump.source.character)
            .collect();

        let expected = vec![
            '^', '^', '^',
            'a', 'a',
            'b'
        ];

        assert_eq!(expected, actual);
    }

    #[test]
    fn jump_destination_characters() {
        // ^ => a | ^ => b | ^ => $
        // a => b | a => $
        // b => $

        let actual: Vec<char> = Jumps::new("ab")
            .map(|jump| jump.destination.character)
            .collect();

        let expected = vec![
            'a', 'b', '$',
            'b', '$',
            '$'
        ];

        assert_eq!(expected, actual);
    }

    #[test]
    fn jump_scores() {
        // ^ => a | ^ => b | ^ => $
        // a => b | a => $
        // b => $

        let actual: Vec<usize> = Jumps::new("ab")
            .map(|jump| jump.score)
            .collect();

        let expected = vec![
            3, 5, 0,
            1, 0,
            0
        ];

        assert_eq!(expected, actual);
    }

    #[test]
    fn mixed_case_jumps() {
        let actual: Vec<char> = Jumps::new("aB")
            .map(|jump| jump.destination.character)
            .collect();

        let expected = vec![
            'a', 'B', 'b', '$',
            'B', 'b', '$',
            '$'
        ];

        assert_eq!(expected, actual);
    }
}
