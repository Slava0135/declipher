pub fn diff(left: &str, right: &str) -> f32 {
    let n = left.chars().count();
    assert_eq!(n, right.chars().count());
    if n == 0 {
        return 0.0;
    }
    let distance = left.chars().zip(right.chars()).fold(0, |acc, (l, r)| {
        if l.is_whitespace() || l == r {
            acc
        } else {
            acc + 1
        }
    });
    return 1.0 - distance as f32 / left.chars().filter(|it| !it.is_whitespace()).count() as f32;
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    const EPSILON: f32 = 0.001;

    #[test]
    fn empty() {
        assert_relative_eq!(0.0, diff("", ""), epsilon = EPSILON);
    }

    #[test]
    fn equal() {
        assert_relative_eq!(1.0, diff("foo bar!", "foo bar!"), epsilon = EPSILON);
    }

    #[test]
    fn not_equal() {
        assert_relative_eq!(0.5, diff("foo! bar!", "boo! foo!"), epsilon = EPSILON);
    }

    #[test]
    fn ignore_space() {
        assert_relative_eq!(0.0, diff("foo bar", "bar foo"), epsilon = EPSILON);
    }
}
