#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}
impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}
// create a iterator
impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        match self.remainder.as_mut() {
            Some(remainder) => {
                if let Some(next_delim) = remainder.find(self.delimiter) {
                    let until_delimiter = &remainder[..next_delim];
                    *remainder = &remainder[(next_delim + self.delimiter.len())..];
                    Some(until_delimiter)
                } else {
                    self.remainder.take()
                }
            }
            None => return None,
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
        assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
        let vecmethod: Vec<&str> = StrSplit::new(haystack, " ").collect();
        assert_eq!(vecmethod, vec!["a", "b", "c", "d", "e"]);
    }
    #[test]
    fn edge_case() {
        let haystack = "a b c d e ";
        // let letters = StrSplit::new(haystack, " ");
        let vecmethod: Vec<&str> = StrSplit::new(haystack, " ").collect();
        assert_eq!(vecmethod, vec!["a", "b", "c", "d", "e", ""]);
    }
}
