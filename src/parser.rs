#![feature(core,collections)]

use nom::{IResult,Needed,not_line_ending};
use nom::IResult::*;
use std::str;

#[derive(Debug,PartialEq,Eq)]
pub struct Base {
  pub key: char,
  pub score: u8,
}

#[derive(Debug,PartialEq,Eq)]
pub struct Reading<'a> {
  pub id: &'a str,
  bases: Vec<Base>,
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

pub fn many_reads(input:&[u8]) -> IResult<&[u8], Vec<Reading>> {
  many1!(input,
    chain!(
      tag!("@") ~
      id: map_res!(not_line_ending, str::from_utf8) ~ tag!("\n") ~
      bases: map_res!(not_line_ending, str::from_utf8) ~ tag!("\n")? ~
      scores: opt!(quality_scores),
      ||{ 
         Reading{
           id: id,
           bases: 
             bases.chars().zip(
               scores.unwrap().chars()
             ).map(|x| Base{key: x.0, score: ('~' as u8) - (x.1 as u8)})
             .collect::<Vec<Base>>()
         }
      }
    )
  )
}

