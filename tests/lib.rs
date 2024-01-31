#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use dedent_macro::dedent;

    #[test]
    fn aligned_lines() {
        let s = dedent!("
          foo
          bar
        ");
        assert_eq!(s, "foo\nbar\n");
    }

    #[test]
    fn indented_lines() {
        let s = dedent!("
          foo
            bar
        ");
        assert_eq!(s, "foo\n  bar\n");
    }

    #[test]
    fn with_lines_full_of_whitespace() {
        // the whitespace in here is 1 space more than bar
        let s = dedent!("
          foo
             
             
            bar
        ");
        assert_eq!(s, "foo\n   \n   \n  bar\n");
    }

    #[test]

    fn a_raw_string_literal() {
        let s = dedent!(r"
          foo
            \
        ");
        assert_eq!(s, r"foo
  \
");
    }

    #[test]
    fn a_raw_string_literal_with_hash_delimiters() {
        let s = dedent!(r###"
          # dedent-macro
          ## Installation
        "###);

        assert_eq!(s, r###"# dedent-macro
## Installation
"###);
    }

}
