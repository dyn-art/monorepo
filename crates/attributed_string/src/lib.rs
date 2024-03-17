use ropey::Rope;
use rust_lapper::{Interval, Lapper};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FontStyle {
    Normal,
    Italic,
    Bold,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Attribute {
    FontSize(u32),
    FontStyle(FontStyle),
    Color(u8, u8, u8),
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
struct Attributes {
    font_size: Option<u32>,
    font_style: Option<FontStyle>,
    color: Option<(u8, u8, u8)>,
}

impl Attributes {
    pub fn update(&mut self, attribute: Attribute) {
        match attribute {
            Attribute::FontSize(size) => self.font_size = Some(size),
            Attribute::FontStyle(style) => self.font_style = Some(style),
            Attribute::Color(r, g, b) => self.color = Some((r, g, b)),
        }
    }
}

type TextRange = Interval<usize, Attributes>;

#[derive(Debug, Clone)]
struct AttributedString {
    text: Rope,
    attribute_intervals: Lapper<usize, Attributes>,
}

impl AttributedString {
    pub fn new(text: &str, attributes: Vec<TextRange>) -> Self {
        Self {
            text: Rope::from_str(text),
            attribute_intervals: Lapper::new(attributes),
        }
    }

    pub fn string(&self) -> String {
        self.text.to_string()
    }

    pub fn length(&self) -> usize {
        self.text.len_chars()
    }

    pub fn attributes(&self, start: usize, stop: usize) -> Vec<Attributes> {
        self.attribute_intervals
            .find(start, stop)
            .map(|interval| interval.val.clone())
            .collect()
    }

    pub fn insert_attribute(&mut self, elem: TextRange) {
        self.attribute_intervals.insert(elem);
    }

    pub fn insert_attributes(&mut self, elems: Vec<TextRange>) {
        for elem in elems {
            self.attribute_intervals.insert(elem);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attribute_insertion_and_query() {
        let mut attributed_string = AttributedString::new("Hello, World!", vec![]);
        let start = 0;
        let stop = 5;
        let attrs = Attributes {
            font_size: Some(14),
            font_style: Some(FontStyle::Bold),
            color: Some((255, 0, 0)),
        };
        let text_range = Interval {
            start,
            stop,
            val: attrs,
        };

        attributed_string.insert_attribute(text_range);

        let query_attrs = attributed_string.attributes(start, stop);
        assert_eq!(query_attrs.len(), 1);
        assert_eq!(query_attrs[0].font_size, Some(14));
        assert_eq!(query_attrs[0].font_style, Some(FontStyle::Bold));
        assert_eq!(query_attrs[0].color, Some((255, 0, 0)));
    }
}
