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
    destination_index: usize
}

impl Jumps {
    pub fn new(path: &str) -> Self {
        let locations = Locations::new(path).collect();

        Jumps {
            locations: locations,
            source_index: 0,
            destination_index: 1
        }
    }
}

impl Iterator for Jumps {
    type Item = Jump;

    fn next(&mut self) -> Option<Jump> {
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
        let jumps: Vec<Jump> = Jumps::new("ab").collect();
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
        let jumps: Vec<Jump> = Jumps::new("ab").collect();
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
        let jumps: Vec<Jump> = Jumps::new("ab").collect();
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
}
