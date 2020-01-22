use pest::iterators::Pairs;
use pest::Parser;
use std::convert::TryFrom;
use std::error;
use std::mem::{self, MaybeUninit};

use crate::hash;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub const DEFAULT_NAMESPACE: u8 = 32;

// TODO:
// Implement default namespace hashing
// Properly support WPP and index multipliers

#[derive(Parser)]
#[grammar = "vw.pest"]
pub struct VWParser;

#[allow(dead_code)]
fn run_len_encode(mut input: u64) -> Vec<u8> {
  let mut buf: Vec<u8> = vec![];
  while input >= 128 {
    buf.push(u8::try_from((input & 127) | 128).unwrap());
    input >>= 7;
  }
  buf.push(u8::try_from(input & 127).unwrap());
  buf
}

#[allow(dead_code)]
fn zigzag_encode(input: i64) -> u64 {
  ((input << 1) as u64) ^ ((input >> 63) as u64)
}

#[derive(Default)]
pub struct FeatureSpace {
  pub values: Vec<(u64, f32)>,
}

pub struct Features {
  pub namespace_indices: Vec<u8>,
  pub feature_spaces: [FeatureSpace; 256],
}

struct NamespaceInfo {
  index: u8,
  name: String,
  hash: u64,
}

impl Features {
  // Turn off this warning because array_of_uninitialized_items cannot be used as a range based for.
  #[allow(clippy::needless_range_loop)]
  pub fn new() -> Features {
    unsafe {
      let uninitialized_array: MaybeUninit<[MaybeUninit<FeatureSpace>; 256]> =
        MaybeUninit::uninit();
      let mut array_of_uninitialized_items: [MaybeUninit<FeatureSpace>; 256] =
        uninitialized_array.assume_init();

      for i in 0..array_of_uninitialized_items.len() {
        array_of_uninitialized_items[i] = MaybeUninit::new(FeatureSpace::new());
      }

      Features {
        namespace_indices: Default::default(),
        feature_spaces: mem::transmute(array_of_uninitialized_items),
      }
    }
  }
}

impl Default for Features {
  fn default() -> Self {
    Self::new()
  }
}

impl FeatureSpace {
  pub fn new() -> FeatureSpace {
    FeatureSpace { values: vec![] }
  }
}

fn to_feature(
  feature_tokens: &mut Pairs<Rule>,
  namespace_hash: u64,
  anonymous_counter: &mut u64,
) -> Result<(u64, f32)> {
  let first_token = feature_tokens.next().unwrap();
  match first_token.as_rule() {
    Rule::name => {
      let feature_name_string = first_token.as_str();
      let feature_name_hash = hash::uniform_hash(feature_name_string.as_bytes(), namespace_hash);
      match feature_tokens.next() {
        Some(token) => Ok((feature_name_hash, token.as_str().parse()?)),
        None => Ok((feature_name_hash, 1.0)),
      }
    }
    Rule::number => {
      *anonymous_counter += 1;
      Ok((
        namespace_hash + (*anonymous_counter - 1),
        first_token.as_str().parse()?,
      ))
    }
    _ => unreachable!(),
  }
}

pub fn parse_line(line: &str, hash_seed: u64) -> Result<Features> {
  let parse_result = VWParser::parse(Rule::line, line)?.next().unwrap();
  process(parse_result.into_inner(), hash_seed)
}

pub fn process(parsed_tokens: Pairs<Rule>, hash_seed: u64) -> Result<Features> {
  let mut features = Features::new();
  for item in parsed_tokens {
    match item.as_rule() {
      Rule::label => {
        println!("label: {}", item.as_str());
      }
      Rule::tag => {
        println!("tag: {}", item.as_str());
      }
      Rule::namespace => {
        println!("namespace: {}", item.as_str());
        let mut namespace_contents = item.into_inner();
        let namespace_info = match namespace_contents.peek() {
          Some(rule) => match rule.as_rule() {
            Rule::name => {
              // The token was the namespace name, consume it now.
              namespace_contents.next().unwrap();

              let name = rule.as_str();
              NamespaceInfo {
                index: name.chars().nth(0).unwrap() as u8,
                name: String::from(name),
                hash: hash::uniform_hash(rule.as_str().as_bytes(), hash_seed),
              }
            }
            // TODO: calculate default hash
            _ => NamespaceInfo {
              index: DEFAULT_NAMESPACE,
              name: String::from(""),
              hash: DEFAULT_NAMESPACE as u64,
            },
          },
          _ => unreachable!(),
        };

        features.namespace_indices.push(namespace_info.index);
        let thing = &mut features.feature_spaces[namespace_info.index as usize].values;

        println!("name: {}", namespace_info.name);
        let mut anonymous_counter = 0;
        for feature_token in namespace_contents {
          thing.push(to_feature(
            &mut feature_token.into_inner(),
            namespace_info.hash,
            &mut anonymous_counter,
          )?);
        }
      }
      Rule::EOI => (),
      _ => unreachable!(),
    }
  }

  Ok(features)
}

#[test]
fn parser_test() {
  let file = VWParser::parse(Rule::line, "4 'tag |test test:-4.5 another -4.5")
    .expect("unsuccessful parse") // unwrap the parse result
    .next()
    .unwrap(); // Get and unwrap the `line` rule; never fail
  let n = file.into_inner();
  let features = process(n, 0).unwrap();
  assert_eq!(features.namespace_indices.len(), 1);
  assert_eq!(
    features.feature_spaces[features.namespace_indices[0] as usize]
      .values
      .len(),
    3
  );
}

#[test]
fn parse_line_test() {
  let features = parse_line("4 'tag |test test:-4.5 another -4.5", 0).unwrap();
  assert_eq!(features.namespace_indices.len(), 1);
  assert_eq!(
    features.feature_spaces[features.namespace_indices[0] as usize]
      .values
      .len(),
    3
  );
}

#[test]
fn zigzag_test() {
  assert_eq!(zigzag_encode(0), 0);
  assert_eq!(zigzag_encode(-1), 1);
  assert_eq!(zigzag_encode(1), 2);
  assert_eq!(zigzag_encode(-2), 3);
  assert_eq!(zigzag_encode(2), 4);
  assert_eq!(zigzag_encode(-3), 5);
  assert_eq!(zigzag_encode(3), 6);
}
