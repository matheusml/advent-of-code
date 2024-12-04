use regex::Regex;

pub fn parse(input: String) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(&input)
        .map(|caps| (caps[1].parse().unwrap(), caps[2].parse().unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(")+*>>?@!mul(657,687)/from()".to_string()),
            vec![(657, 687)]
        );
    }

    #[test]
    fn test_parse_multiple() {
        assert_eq!(
            parse("why()mul(725,478)( who(794,427)&><mul(772,630)^$how()?why()$%from()who()mul(58,324)".to_string()),
            vec![(725, 478), (772, 630), (58, 324)]
        );
    }
}
