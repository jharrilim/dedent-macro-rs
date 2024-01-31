use litrs::{ByteStringLit, StringLit};
use proc_macro::TokenStream;
use proc_macro_error::{proc_macro_error, OptionExt};
use quote::ToTokens;

#[derive(Debug, Clone, PartialEq)]
struct Lines {
    lines: Vec<Line>,
    least_spaces: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct Line {
    line: String,
    leading_space_count: i32,
}

/// Use this macro to dedent string literals.
/// 
/// # Examples
/// 
/// ```
/// use dedent_macro::dedent;
/// fn foo() {
///   let s = dedent!("
///     Hello,
///     World!
///   ");
/// }
/// 
/// ```
#[proc_macro]
#[proc_macro_error]
pub fn dedent(input: TokenStream) -> TokenStream {
    let first_arg = input
        .into_iter()
        .next()
        .expect_or_abort("Expected a string literal but found nothing.");

    let mut lines = {
        let s = StringLit::try_from(first_arg.clone())
            .map_err(|_| ByteStringLit::try_from(first_arg))
            .map(|s| s.value().to_string())
            .ok()
            .expect_or_abort("Expected a string literal");
        analyze(&s)
    };

    lines.lines.iter_mut().for_each(|l| {
        l.line = remove_space(l.line.clone(), lines.least_spaces);
    });

    lines
        .lines
        .into_iter()
        .map(|l| l.line)
        .collect::<Vec<String>>()
        .join("\n")
        .into_token_stream()
        .into()
}

fn analyze(s: &str) -> Lines {
    let s = s
        .trim_start_matches('"')
        .trim_end_matches('"')
        .trim_start_matches('\n');
    let mut least_spaces: Option<i32> = None;
    let mut lines: Vec<Line> = s
        .lines()
        .map(|l| {
            let mut space_count = 0;
            let line = l.to_string();

            for b in line.chars() {
                if b == ' ' {
                    space_count += 1;
                } else {
                    break;
                }
            }

            match least_spaces {
                None => least_spaces = Some(space_count),
                Some(s) => {
                    least_spaces = if s < space_count
                        || line.is_empty()
                        || line.chars().all(|c| c.is_whitespace())
                    {
                        least_spaces
                    } else {
                        Some(space_count)
                    };
                }
            }

            Line {
                leading_space_count: space_count,
                line,
            }
        })
        .collect();

    Lines {
        lines: match lines.last_mut() {
            Some(line) => {
                if line.line.chars().all(|c| c.is_whitespace()) {
                    line.line = "".to_string();
                    lines
                } else {
                    lines
                }
            }
            None => lines,
        },
        least_spaces: least_spaces.unwrap(),
    }
}

fn remove_space(string: String, spaces: i32) -> String {
    let trim_these = " ".repeat(spaces as usize);
    string.trim_start_matches(&trim_these).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn dedent_a_string() {
        let s = analyze("
          foo
            bar
        ");

        assert_eq!(s.least_spaces, 10);
        assert_eq!(s.lines.len(), 3);
        assert_eq!(s.lines[0].leading_space_count, 10);
        assert_eq!(s.lines[1].leading_space_count, 12);
        assert_eq!(s.lines[2].line.as_str(), "");
    }
}
