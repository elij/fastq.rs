#[macro_use]
extern crate nom;
extern crate fastq;


mod tests {
  use nom::IResult;
  use fastq::many_reads;

  #[test]
  fn should_parse_two_reads() {
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
    match reads {
      IResult::Done(_, o) => assert_eq!(header, o[0].id),
      _ => assert!(false)
    }
  }

  #[test]
  fn should_parse_one_read_without_newline() {
    let header = "SRR316957.252299 B086EABXX110425:3:1102:20344:169356/1";
    let seq =
&b"@SRR316957.252299 B086EABXX110425:3:1102:20344:169356/1
CAGACACAAACCTTTCTTTGTGTGGAGCTCCCACGGTAAAAAGACCATTGTCAAGTGCATGTATATAGGTTCCCTC
+
DFBDFBG@DGBE=EBFFFEFFFFCF/9>89@??:?CCDCCGG=EEDEBF8?@BBC?D;EDEACEEEEBED?=FGGF"[..];

    let reads = many_reads(seq);
    match reads {
      IResult::Done(_, o) => assert_eq!(header, o[0].id),
      _ => assert!(false)
    }
  }
}

