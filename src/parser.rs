use std::str;
use nom::IResult::*;
use nom::{IResult,Needed,not_line_ending,line_ending,eof};

#[derive(Debug)]
pub struct Base {
  pub key: Nucleobase,
  pub score: f32,
}

#[derive(Debug)]
pub struct Sequence<'a> {
  pub id: &'a str,
  pub bases: Vec<Base>,
}

#[derive(Debug,PartialEq,Eq)]
pub enum Nucleobase {
  A,
  C,
  G,
  T,
  U,
  N
}

pub fn quality_scores(input:&[u8]) -> IResult<&[u8], &str> {
  chain!(input,
    tag!("+") ~
    line_ending ~
    scores: map_res!(not_line_ending, str::from_utf8) ~
    alt!(eof | line_ending),
    ||{scores}
  )
}

pub fn many_reads(input:&[u8]) -> IResult<&[u8], Vec<Sequence>> {
  many1!(input,
    chain!(
      tag!("@") ~
      id: map_res!(not_line_ending, str::from_utf8) ~ line_ending ~
      bases: map_res!(not_line_ending, str::from_utf8) ~ alt!(line_ending | eof) ~
      scores: opt!(quality_scores),
      ||{ 
         Sequence {
           id: id,
           bases: 
             bases.chars().zip(
               scores.unwrap().chars()
             ).map(|x| Base{key:
               match x.0 {
                 'A' => Nucleobase::A,
                 'C' => Nucleobase::C,
                 'G' => Nucleobase::G,
                 'T' => Nucleobase::T,
                 'U' => Nucleobase::U,
                 _ => Nucleobase::N,
               }
             , score: (('~' as u8) - (x.1 as u8)) as f32})
             .collect::<Vec<Base>>()
         }
      }
    )
  )
}
