#[allow(unused_imports)]
use crate::dom::{AttrMap, Element, Node, Text};
use combine::error::ParseError;
use combine::error::StringStreamError;
use combine::parser::char::char;
#[allow(unused_imports)]
use combine::EasyParser;
use combine::{parser, Parser, Stream};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum HTMLParseError {
    #[error("failed to parse; {0}")]
    InvalidResourceError(StringStreamError),
}

// [NOTE] Specification on HTML parsing: https://html.spec.whatwg.org/multipage/parsing.html#parsing
//
// The specification defines parsing algorithm of HTML, which takes input stream as argument and emits DOM.
// It consists of the following two stages:
// 1. tokenization stage
// 2. tree construction stage
// The first one, tokenization stage, generates tokens from input stream.
// The latter one, tree construction stage, constructs a DOM while handling scripts inside <script> tags.
//
// This implementation omits details of those two stages for simplicity.
// Please check the following if you'd like to know about the parsing process more deeply:
// - html5ever crate by Serve project https://github.com/servo/html5ever
// - HTMLDocumentParser, HTMLTokenizer, HTMLTreeBuilder of Chromium (src/third_party/blink/renderer/core/html/parser/*)

// `nodes_` (and `nodes`) tries to parse input as Element or Text.
fn nodes_<Input>() -> impl Parser<Input, Output = Vec<Box<Node>>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    todo!("you need to implement this combinator");
    (char(' ')).map(|_| vec![Element::new("".into(), AttrMap::new(), vec![])])
}

/// `text` consumes input until `<` comes.
fn text<Input>() -> impl Parser<Input, Output = Box<Node>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    todo!("you need to implement this combinator");
    (char(' ')).map(|_| Element::new("".into(), AttrMap::new(), vec![]))
}

/// `element` consumes `<tag_name attr_name="attr_value" ...>(children)</tag_name>`.
fn element<Input>() -> impl Parser<Input, Output = Box<Node>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    todo!("you need to implement this combinator");
    (char(' ')).map(|_| Element::new("".into(), AttrMap::new(), vec![]))
}

/// `open_tag` consumes `<tag_name attr_name="attr_value" ...>`.
fn open_tag<Input>() -> impl Parser<Input, Output = (String, AttrMap)>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    todo!("you need to implement this combinator");
    (char(' ')).map(|_| ("".to_string(), AttrMap::new()))
}

/// close_tag consumes `</tag_name>`.
fn close_tag<Input>() -> impl Parser<Input, Output = String>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    todo!("you need to implement this combinator");
    (char(' ')).map(|_| ("".to_string()))
}

/// `attribute` consumes `name="value"`.
fn attribute<Input>() -> impl Parser<Input, Output = (String, String)>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    todo!("you need to implement this combinator");
    (char(' ')).map(|_| ("".to_string(), "".to_string()))
}

/// `attributes` consumes `name1="value1" name2="value2" ... name="value"`
fn attributes<Input>() -> impl Parser<Input, Output = AttrMap>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    todo!("you need to implement this combinator");
    (char(' ')).map(|_| AttrMap::new())
}

parser! {
    fn nodes[Input]()(Input) -> Vec<Box<Node>>
    where [Input: Stream<Token = char>]
    {
        nodes_()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // parsing tests of attributes
    #[test]
    fn test_parse_attribute() {
        assert_eq!(
            attribute().easy_parse("test=\"foobar\""),
            Ok((("test".to_string(), "foobar".to_string()), ""))
        );

        assert_eq!(
            attribute().easy_parse("test = \"foobar\""),
            Ok((("test".to_string(), "foobar".to_string()), ""))
        )
    }

    #[test]
    fn test_parse_attributes() {
        let mut expected_map = AttrMap::new();
        expected_map.insert("test".to_string(), "foobar".to_string());
        expected_map.insert("abc".to_string(), "def".to_string());
        assert_eq!(
            attributes().easy_parse("test=\"foobar\" abc=\"def\""),
            Ok((expected_map, ""))
        );

        assert_eq!(attributes().easy_parse(""), Ok((AttrMap::new(), "")))
    }

    #[test]
    fn test_parse_open_tag() {
        {
            assert_eq!(
                open_tag().easy_parse("<p>aaaa"),
                Ok((("p".to_string(), AttrMap::new()), "aaaa"))
            );
        }
        {
            let mut attributes = AttrMap::new();
            attributes.insert("id".to_string(), "test".to_string());
            assert_eq!(
                open_tag().easy_parse("<p id=\"test\">"),
                Ok((("p".to_string(), attributes), ""))
            )
        }

        {
            let result = open_tag().easy_parse("<p id=\"test\" class=\"sample\">");
            let mut attributes = AttrMap::new();
            attributes.insert("id".to_string(), "test".to_string());
            attributes.insert("class".to_string(), "sample".to_string());
            assert_eq!(result, Ok((("p".to_string(), attributes), "")));
        }

        {
            assert!(open_tag().easy_parse("<p id>").is_err());
        }
    }

    // parsing tests of close tags
    #[test]
    fn test_parse_close_tag() {
        let result = close_tag().easy_parse("</p>");
        assert_eq!(result, Ok(("p".to_string(), "")))
    }

    #[test]
    fn test_parse_element() {
        assert_eq!(
            element().easy_parse("<p></p>"),
            Ok((Element::new("p".to_string(), AttrMap::new(), vec![]), ""))
        );

        assert_eq!(
            element().easy_parse("<p>hello world</p>"),
            Ok((
                Element::new(
                    "p".to_string(),
                    AttrMap::new(),
                    vec![Text::new("hello world".to_string())]
                ),
                ""
            ))
        );

        assert_eq!(
            element().easy_parse("<div><p>hello world</p></div>"),
            Ok((
                Element::new(
                    "div".to_string(),
                    AttrMap::new(),
                    vec![Element::new(
                        "p".to_string(),
                        AttrMap::new(),
                        vec![Text::new("hello world".to_string())]
                    )],
                ),
                ""
            ))
        );

        assert!(element().easy_parse("<p>hello world</div>").is_err());
    }

    #[test]
    fn test_parse_text() {
        {
            assert_eq!(
                text().easy_parse("Hello World"),
                Ok((Text::new("Hello World".to_string()), ""))
            );
        }
        {
            assert_eq!(
                text().easy_parse("Hello World<"),
                Ok((Text::new("Hello World".to_string()), "<"))
            );
        }
    }
}