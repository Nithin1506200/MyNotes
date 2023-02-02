use std::str::FromStr;

struct Equation {
    left: String,
    right: String,
    is_equal: bool,
}
struct ParseEquationError;
impl FromStr for Equation {
    type Err = ParseEquationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once("=")
            .and_then(|(l, r)| {
                let left = sum(l);
                let right = sum(r);
                Some(Equation {
                    left: l.trim().to_string(),
                    right: r.trim().to_string(),
                    is_equal: left == right,
                })
            })
            .ok_or(ParseEquationError)
    }
}
impl ToString for Equation {
    fn to_string(&self) -> String {
        format!("{} {} {}", self.left, self.right, self.is_equal)
    }
}
fn sum(s: &str) -> usize {
    s.split('+')
        .map(|x| x.trim().parse::<usize>().unwrap_or(0))
        .sum()
}
#[cfg(test)]
mod test {
    use super::Equation;
    #[test]
    fn parse_test() {
        let s = "nithin=jerin";
        match s.parse::<Equation>() {
            Ok(Equation {
                left,
                right,
                is_equal,
            }) => {
                println!("{} {} {} {}", left, right, is_equal, left == right);
            }
            Err(_) => print!("boom"),
        }
    }

    #[test]
    fn create_str() {
        let s = Equation {
            left: "nithin".into(),
            right: "jerin".into(),
            is_equal: false,
        };
        println!("{}", s.to_string())
    }
}
