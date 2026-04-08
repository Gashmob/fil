// fil
// Copyright (C) 2026 - Present  fil contributors
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

use lalrpop_util::ParseError;
use lalrpop_util::lexer::Token;

pub fn format_parse_error(error: ParseError<usize, Token, &str>, source: &String) -> String {
    let result = match error {
        ParseError::InvalidToken { location } => parse_invalid_token(location, source),
        ParseError::UnrecognizedEof {
            location,
            ref expected,
        } => parse_unrecognized_eof(location, expected, source),
        ParseError::UnrecognizedToken {
            ref token,
            ref expected,
        } => parse_unrecognized_token(token, expected, source),
        ParseError::ExtraToken { ref token } => parse_extra_token(token, source),
        ParseError::User { error } => Some(String::from(error)),
    };

    result.unwrap_or(format!("{error}"))
}

fn parse_invalid_token(location: usize, source: &String) -> Option<String> {
    find_line(location, location, source)
        .map(|(line, n)| format!("Invalid token at line {n}:\n\n{line}"))
}

fn parse_unrecognized_eof(
    location: usize,
    expected: &Vec<String>,
    source: &String,
) -> Option<String> {
    find_line(location, location, source).map(|(line, n)| {
        let expected_str = format_expected(expected);
        format!("Unexpected end of file (EOF) at line {n}:\n\n{line}\n{expected_str}\n")
    })
}

fn parse_unrecognized_token(
    (start, token, end): &(usize, Token, usize),
    expected: &Vec<String>,
    source: &String,
) -> Option<String> {
    find_line(*start, *end, source).map(|(line, n)| {
        let expected_str = format_expected(expected);
        format!("Unexpected token '{token}' at line {n}:\n\n{line}\n{expected_str}\n")
    })
}

fn parse_extra_token(
    (start, token, end): &(usize, Token, usize),
    source: &String,
) -> Option<String> {
    find_line(*start, *end, source)
        .map(|(line, n)| format!("Extra token '{token}' found at line {n}:\n\n{line}\n"))
}

fn format_expected(expected: &Vec<String>) -> String {
    let mut result = String::new();
    for (i, e) in expected.iter().enumerate() {
        let sep = match i {
            0 => "Expected",
            _ if i < expected.len() - 1 => ",",
            _ => " or",
        };
        result = format!("{result}{sep} '{e}'");
    }
    result
}

struct LineEntry {
    line: String,
    nth_line: usize,
    range: (usize, usize),
}

fn find_line(start: usize, end: usize, source: &String) -> Option<(String, usize)> {
    let lines = collect_lines(start, end, source);
    if lines.is_empty() {
        None
    } else if lines.len() == 1 {
        let line = lines.first()?;
        let n = line.nth_line;
        let content = &line.line;
        let n_spacing = " ".repeat(format!("{n}").len());
        let spacing = " ".repeat(line.range.0);
        let hats = "^".repeat(line.range.1 - line.range.0 + 1);
        Some((
            format!(" {n} | {content}\n {n_spacing}   {spacing}{hats}\n"),
            n,
        ))
    } else {
        let first_line = lines.first()?;
        let last_line = lines.last()?;
        let n_len = format!("{}", last_line.nth_line).len();
        let n_spacing = " ".repeat(n_len);
        let mut content = String::new();
        for (i, line) in lines.iter().enumerate() {
            let n = line.nth_line;
            let front_spacing = " ".repeat(n_len - format!("{n}").len());
            let line_content = &line.line;
            if i == 0 {
                let spacing = " ".repeat(line.range.0);
                content +=
                    format!(" {n_spacing}   {spacing}v\n {front_spacing}{n} | {line_content}\n")
                        .as_str();
            } else if i == lines.len() - 1 {
                let spacing = " ".repeat(line.range.1);
                content +=
                    format!(" {front_spacing}{n} | {line_content}\n {n_spacing}   {spacing}^\n")
                        .as_str();
            } else {
                content += format!(" {front_spacing}{n} | {line_content}\n").as_str();
            }
        }
        Some((content, first_line.nth_line))
    }
}

fn collect_lines(start: usize, end: usize, source: &String) -> Vec<LineEntry> {
    let mut lines = Vec::new();
    let mut position_start = 0;
    let mut position_end = 0;
    let mut collect = false;
    for (n, line) in source.split("\n").enumerate() {
        position_end += line.len() + 1;
        let mut line_start = 0;
        let mut line_end = line.len();
        if start >= position_start && start < position_end {
            line_start = start - position_start;
            collect = true;
        }
        if collect {
            if end >= position_start && end < position_end {
                line_end = end - position_start;
            }
            lines.push(LineEntry {
                line: String::from(line),
                nth_line: n + 1,
                range: (line_start, line_end),
            });
        }
        if end >= position_start && end < position_end {
            collect = false;
        }

        position_start = position_end;
    }
    lines
}

#[cfg(test)]
mod test {
    use crate::build::parse_error_formatter::{find_line, format_expected, format_parse_error};
    use lalrpop_util::ParseError;
    use lalrpop_util::lexer::Token;
    use pretty_assertions::{assert_eq, assert_str_eq};

    #[test]
    fn test_parse_invalid_token() {
        assert_str_eq!(
            "Invalid token at line 2:

 2 | bar
      ^
",
            format_parse_error(
                ParseError::InvalidToken { location: 5 },
                &String::from("foo\nbar\nbaz")
            )
        );
        assert_str_eq!(
            "Invalid token at 50",
            format_parse_error(
                ParseError::InvalidToken { location: 50 },
                &String::from("foo\nbar\nbaz")
            )
        )
    }

    #[test]
    fn test_parse_unrecognized_eof() {
        assert_str_eq!(
            "Unexpected end of file (EOF) at line 3:

 3 | baz
        ^

Expected 'toto' or 'titi'
",
            format_parse_error(
                ParseError::UnrecognizedEof {
                    location: 11,
                    expected: vec![String::from("toto"), String::from("titi")],
                },
                &String::from("foo\nbar\nbaz"),
            )
        );
    }

    #[test]
    fn test_parse_unrecognized_token() {
        assert_str_eq!(
            "Unexpected token 'hello' at line 2:

 2 | bar
      ^^

Expected 'toto' or 'titi'
",
            format_parse_error(
                ParseError::UnrecognizedToken {
                    token: (5, Token(0, "hello"), 6),
                    expected: vec![String::from("toto"), String::from("titi")],
                },
                &String::from("foo\nbar\nbaz"),
            ),
        );
    }

    #[test]
    fn test_parse_user() {
        assert_str_eq!(
            "factoring",
            format_parse_error(ParseError::User { error: "factoring" }, &String::new(),)
        )
    }

    #[test]
    fn test_parse_extra_token() {
        assert_str_eq!(
            "Extra token 'hello' found at line 2:

 2 | bar
      ^^

",
            format_parse_error(
                ParseError::ExtraToken {
                    token: (5, Token(0, "hello"), 6)
                },
                &String::from("foo\nbar\nbaz"),
            ),
        )
    }

    #[test]
    fn test_format_expected() {
        assert_str_eq!(
            "Expected 'foo'",
            format_expected(&vec![String::from("foo")])
        );
        assert_str_eq!(
            "Expected 'foo' or 'bar'",
            format_expected(&vec![String::from("foo"), String::from("bar")])
        );
        assert_str_eq!(
            "Expected 'foo', 'bar' or 'baz'",
            format_expected(&vec![
                String::from("foo"),
                String::from("bar"),
                String::from("baz")
            ])
        );
    }

    #[test]
    fn test_find_line() {
        assert_eq!(
            Some((
                String::from(
                    " 2 | bar
      ^
"
                ),
                2
            )),
            find_line(5, 5, &String::from("foo\nbar\nbaz")),
        );
        assert_eq!(
            Some((
                String::from(
                    " 1 | foo
     ^
"
                ),
                1
            )),
            find_line(0, 0, &String::from("foo\nbar\nbaz")),
        );
        assert_eq!(
            Some((
                String::from(
                    " 1 | foo
        ^
"
                ),
                1
            )),
            find_line(3, 3, &String::from("foo\nbar\nbaz")),
        );
        assert_eq!(
            Some((
                String::from(
                    " 3 | baz
       ^
"
                ),
                3
            )),
            find_line(10, 10, &String::from("foo\nbar\nbaz")),
        );
        assert_eq!(None, find_line(100, 100, &String::from("")));
    }

    #[test]
    fn test_find_line_range() {
        assert_eq!(
            Some((
                String::from(
                    " 2 | Hello World!
           ^^^^^
"
                ),
                2
            )),
            find_line(10, 14, &String::from("foo\nHello World!\nbaz")),
        );
    }

    #[test]
    fn test_find_line_multiline() {
        assert_eq!(
            Some((
                String::from(
                    "       v
 2 | tete
 3 | titi
 4 | toto
      ^
"
                ),
                2
            )),
            find_line(7, 16, &String::from("tata\ntete\ntiti\ntoto\ntutu\n")),
        );
        assert_eq!(
            Some((
                String::from(
                    "      v
  9 | i
 10 | j
 11 | k
      ^
"
                ),
                9
            )),
            find_line(
                16,
                20,
                &String::from(
                    "a\nb\nc\nd\ne\nf\ng\nh\ni\nj\nk\nl\nm\nn\no\np\nq\nr\ns\nt\nu\nv\nw\nx\ny\nz"
                )
            ),
        );
    }
}
