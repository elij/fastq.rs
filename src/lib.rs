#![feature(core,collections)]

#[macro_use]
extern crate nom;

pub mod parser;

mod tests {
  use super::*;
  use nom::IResult;
  use super::parser::*;  

  #[test]
  fn test1() {
    let header = "SRR316957.252299 B086EABXX110425:3:1102:20344:169356/1";
    let seq =
&b"@SRR316957.252299 B086EABXX110425:3:1102:20344:169356/1
CAGACACAAACCTTTCTTTGTGTGGAGCTCCCACGGTAAAAAGACCATTGTCAAGTGCATGTATATAGGTTCCCTC
+
DFBDFBG@DGBE=EBFFFEFFFFCF/9>89@??:?CCDCCGG=EEDEBF8?@BBC?D;EDEACEEEEBED?=FGGF
@SRR316957.252299 B086EABXX110425:3:1102:20344:169356/1
CAGACACAAACCTTTCTTTGTGTGGAGCTCCCACGGTAAAAAGACCATTGTCAAGTGCATGTATATAGGTTCCCTC
+
DFBDFBG@DGBE=EBFFFEFFFFCF/9>89@??:?CCDCCGG=EEDEBF8?@BBC?D;EDEACEEEEBED?=FGGF
"[..];

    let reads = many_reads(seq);
    println!("{:?}", reads);  
    match reads {
      IResult::Done(_, o) => assert_eq!(header, o[0].id),
      _ => assert!(false)
    }
  }
}

