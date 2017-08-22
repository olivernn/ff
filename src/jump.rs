use location::{Location, Locations, LocationLabel};

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
        }

        // ending on a word begin is only slightly more expensive
        (_, LocationLabel::WordBegin) => {
            score + 3
        },

        _ => {
            // if the locations are contiguous then its _very_ cheap
            if (destination.index - source.index == 1) {
                score + 1
            
            // otherwise its expensive
            } else {
                score + 5
            }
        }
    }
}