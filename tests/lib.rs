#[cfg(test)]
mod tests {
    use dedent_macro::dedent;

    #[test]
    fn aligned_lines() {
        let s = dedent!("
          foo
          bar
        ");
        assert_eq!(s, "foo\nbar");
    }

    #[test]
    fn indented_lines() {
        let s = dedent!("
          foo
            bar
        ");
        assert_eq!(s, "foo\n  bar");
    }

    #[test]
    fn with_lines_full_of_whitespace() {
        // the whitespace in here is 1 space more than bar
        let s = dedent!("
          foo
             
             
            bar
        ");
        assert_eq!(s, "foo\n   \n   \n  bar");
    }
}