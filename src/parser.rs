#![feature(core,collections)]

use nom::{IResult,Needed,not_line_ending};
use nom::IResult::*;
use std::str;

#[derive(Debug,PartialEq,Eq)]
pub struct Base {
  pub key: Nucleobase,
  pub score: u8,
}

#[derive(Debug,PartialEq,Eq)]
pub struct Sequence<'a> {
  pub id: &'a str,
  bases: Vec<Base>,
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
    tag!("\n") ~
    scores: map_res!(not_line_ending, str::from_utf8) ~
    tag!("\n")?,
    ||{scores}
  )
}

pub fn many_reads(input:&[u8]) -> IResult<&[u8], Vec<Sequence>> {
  many1!(input,
    chain!(
      tag!("@") ~
      id: map_res!(not_line_ending, str::from_utf8) ~ tag!("\n") ~
      bases: map_res!(not_line_ending, str::from_utf8) ~ tag!("\n")? ~
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
             , score: ('~' as u8) - (x.1 as u8)})
             .collect::<Vec<Base>>()
         }
      }
    )
  )
}

