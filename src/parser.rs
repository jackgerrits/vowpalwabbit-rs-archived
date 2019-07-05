use nom::{
  IResult,
  bytes::complete::{tag, take_till, is_not},
  combinator::map,
  combinator::opt,
  sequence::tuple,
  character::complete::multispace0,
  number::complete::float,
  character::complete::*,
};

fn label_section(input: &str) -> IResult<&str, (Option<f32>, Option<&str>) > {
  let (input, _) = multispace0(input)?;
  let (input, label_tuple) = opt(tuple((float,multispace1)))(input)?;
  let (input, tag_tuple) = opt(tuple((opt(tag("'")), is_not("|"))))(input)?;

  let label = match label_tuple
  {
    Some((label, _)) => Some(label),
    None => None
  };

  let tag = match tag_tuple
  {
    Some((_, tag)) => Some(tag),
    None => None
  };

  Ok((input, (label, tag)))
}

fn parse_feature<'a>(s: &'a str) -> IResult<&'a str, (&'a str, &'a str)> {
  let (input, feature_name) = take_till(|c| c == ':')(s)?;
  let (input, _) = tag(":")(input)?;
  let (input, feature_value) = take_till(|c| c == ' ')(input)?;

  Ok((input, (feature_name, feature_value)))
}

fn parse_namespace<'a>(s: &'a str) -> IResult<&'a str, (Option<&'a str>, Vec<(&'a str, &'a str)>)> {
  let (input, ns) = opt(is_not(" "))(s)?;
  let (input, _) = multispace0(input)?;
  let (input, features) = nom::multi::separated_list(space1, parse_feature)(input)?;
  let (input, _) = multispace0(input)?;
  Ok((input, (ns, features)))
}

fn split_line_into_sections<'a>(s: &'a str) -> IResult<&'a str, Vec<(Option<&'a str>)>> {
  let (left, mut sections) =
    nom::multi::many0(map(tuple((opt(is_not("|")), tag("|"))), |(option, _)| {
      option
    }))(s)?;
  sections.push(Some(left));
  Ok(("", sections))
}

#[derive(PartialEq, Debug)]
struct Example<'a>{
  pub label: Option<f32>,
  pub tag: Option<&'a str>,
  pub namespaces: Vec<(Option<&'a str>, Vec<(&'a str, &'a str)>)>
}

fn parse_example<'a>(s: &'a str) -> IResult<&'a str, Example<'a>> {
  let (_, sections) = split_line_into_sections(s)?;
  let (label, tag) = match sections[0] {
    Some(string) => match label_section(string)? {
      (_, (label, tag)) => (label, tag),
    },
    None => (None, None),
  };
  let mut namespaces: Vec<(Option<&str>, Vec<(&str, &str)>)> = vec![];
  for section in sections.iter().skip(1) {
    match section {
      Some(section) => {
        let (_, sec) = parse_namespace(section)?;
        namespaces.push(sec);
      }
      None => (),
    }
  }
  Ok(("", Example{label, tag, namespaces}))
}

#[test]
fn parse_label_section() {
  assert_eq!(label_section("0.3 "), Ok(("", (Some(0.3), None))));
  assert_eq!(label_section("  0.6 "), Ok(("", (Some(0.6), None))));
  assert_eq!(label_section("  0.6  tag"), Ok(("", (Some(0.6), Some("tag")))));
  assert_eq!(label_section("  0.6  'tag"), Ok(("", (Some(0.6), Some("tag")))));
  assert_eq!(label_section("tag"), Ok(("", (None, Some("tag")))));
  assert_eq!(label_section("'tag"), Ok(("", (None, Some("tag")))));
}

#[test]
fn parse_example_test() {
  assert_eq!(parse_example("0.3 | test:1"), Ok(("", Example{label: Some(0.3), tag: None, namespaces:vec![(None, vec![("test", "1")])]})));
}

#[test]
fn parse_namespace_test() {
  assert_eq!(
    parse_namespace("             test:one   test:two          "),
    Ok(("", (None, vec![("test", "one"), ("test", "two")])))
  );
  assert_eq!(
    parse_namespace("             test:one   test:two          "),
    Ok(("", (None, vec![("test", "one"), ("test", "two")])))
  );
}

#[test]
fn split_line_into_sections_test() {
  assert_eq!(
    split_line_into_sections(" |             test:one   test:two     |    test:one   test:two  "),
    Ok((
      "",
      vec![
        Some(" "),
        Some("             test:one   test:two     "),
        Some("    test:one   test:two  ")
      ]
    ))
  );
}