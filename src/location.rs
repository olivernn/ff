use unicode_segmentation::{UnicodeSegmentation, GraphemeIndices};

use std::slice::Iter;
use std::str::CharIndices;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum LocationLabel {
    Simple,
    WordBegin,
    PathBegin,
    PathEnd
}

#[derive(Clone)]
pub struct Location {
    pub index: usize,
    pub height: usize,
    pub character: char,
    pub prefix: String,
    pub label: LocationLabel
}

pub struct Locations<'a> {
    chars: CharIndices<'a>,
    previous: Option<char>,
    height_map: HashMap<usize, usize>,
    source: &'a str,
    path_begin: Option<Location>,
    path_end: Option<Location>
}

impl<'a> Locations<'a> {
    pub fn new(source: &'a str) -> Self {
        let chars = (&source).char_indices();
        let mut height_map: HashMap<usize, usize> = HashMap::new();

        let mut height = source.match_indices("/").collect::<Vec<_>>().len();

        for (index, c) in chars.clone() {
            if c == '/' {
                height = height - 1;
            }

            height_map.insert(index, height);
        }

        let path_begin = Location {
            index: 0,
            character: '^',
            label: LocationLabel::PathBegin,
            height: 0,
            prefix: String::from("")
        };

        let path_end = Location {
            index: source.len(),
            character: '$',
            label: LocationLabel::PathEnd,
            height: 0,
            prefix: source.to_owned()
        };

        Locations {
            chars: chars,
            previous: None,
            height_map: height_map,
            source: source,
            path_begin: Some(path_begin),
            path_end: Some(path_end)
        }
    }

    fn path_begin(&self) -> Location {
        Location {
            index: 0,
            character: '^',
            label: LocationLabel::PathBegin,
            height: 0,
            prefix: String::from("")
        }
    }

    fn path_end(&self) -> Location {
        Location {
            index: self.source.len(),
            character: '$',
            label: LocationLabel::PathEnd,
            height: 0,
            prefix: self.source.to_owned()
        }
    }
}

impl<'a> Iterator for Locations<'a> {
    type Item = Location;

    fn next(&mut self) -> Option<Location> {
        if (self.path_begin.is_some()) {
            return self.path_begin.take();
        }

        self.chars.next().and_then(|(index, character)| {
            let location = self.previous.as_ref()
                .map(|previous_character| {

                    let label = match (previous_character, character) {
                        (_, a) if a.is_uppercase() => LocationLabel::WordBegin,
                        (&'_', _) => LocationLabel::WordBegin,
                        (&'-', _) => LocationLabel::WordBegin,
                        (&'/', _) => LocationLabel::WordBegin,
                        _ => LocationLabel::Simple
                    };

                    Location {
                        index: index,
                        character: character,
                        label: label,
                        height: self.height_map[&index],
                        prefix: self.source[..index].to_owned()
                    }
                })
                .or_else(|| {
                    Some(Location {
                        index: index,
                        character: character,
                        label: LocationLabel::WordBegin,
                        height: self.height_map[&index],
                        prefix: self.source[..index].to_owned()
                    })
                });
            
            self.previous = Some(character);

            return location;
        }).or_else(|| {
            self.path_end.take()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let locations: Vec<Location> = Locations::new("ab").collect();

        assert_eq!(locations[1].index, 0);
        assert_eq!(locations[1].character, 'a');
        assert_eq!(locations[1].label, LocationLabel::WordBegin);
        assert_eq!(locations[1].height, 0);
        assert_eq!(locations[1].prefix, "");

        assert_eq!(locations[2].index, 1);
        assert_eq!(locations[2].character, 'b');
        assert_eq!(locations[2].label, LocationLabel::Simple);
        assert_eq!(locations[2].height, 0);
        assert_eq!(locations[2].prefix, "a");
    }

    #[test]
    fn underscore_word_begin() {
        let locations: Vec<Location> = Locations::new("a_b").collect();

        assert_eq!(locations[3].index, 2);
        assert_eq!(locations[3].character, 'b');
        assert_eq!(locations[3].label, LocationLabel::WordBegin);
        assert_eq!(locations[3].height, 0);
        assert_eq!(locations[3].prefix, "a_");
    }

    #[test]
    fn hyphen_word_begin() {
        let locations: Vec<Location> = Locations::new("a-b").collect();

        assert_eq!(locations[3].index, 2);
        assert_eq!(locations[3].character, 'b');
        assert_eq!(locations[3].label, LocationLabel::WordBegin);
        assert_eq!(locations[3].height, 0);
        assert_eq!(locations[3].prefix, "a-");
    }

    #[test]
    fn camel_word_begin() {
        let locations: Vec<Location> = Locations::new("aB").collect();

        assert_eq!(locations[2].index, 1);
        assert_eq!(locations[2].character, 'B');
        assert_eq!(locations[2].label, LocationLabel::WordBegin);
        assert_eq!(locations[2].height, 0);
        assert_eq!(locations[2].prefix, "a");
    }

    #[test]
    fn slash_word_begin() {
        let locations: Vec<Location> = Locations::new("a/b").collect();

        assert_eq!(locations[3].index, 2);
        assert_eq!(locations[3].character, 'b');
        assert_eq!(locations[3].label, LocationLabel::WordBegin);
        assert_eq!(locations[3].height, 0);
        assert_eq!(locations[3].prefix, "a/");
    }
}