pub struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}
impl StrSplit<'_> {
    pub fn new(haystack: &str, delimiter: &str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}
// create a iterator
impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else if self.remainder.is_empty() {
            None
        } else {
            let rest = self.remainder;
            self.remainder = &[];
            Some(rest)
        }
    }
}

#[cfg(test)]
mod test {
    use super::StrSplit;

    #[test]
    fn it_works() {
        let haystack = "a b c d e";
        let letters = StrSplit::new(haystack, " ");
        assert_eq!(letters, vec!["a", "b", "c", "d", "e"].into_iter());
    }
}
