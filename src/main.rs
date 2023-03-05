use std::{io::{Read, stdin, stdout, Write}, fs::File, str::Chars};
use anyhow::{Result, anyhow};
use atty::Stream;
use clap::Parser;

#[derive(Parser, Debug, Default)]
#[clap(author="rh", version, about="convert binary from hex-string.")]
struct Args {
    #[arg(short, long, value_name="in")]
    in_path: Option<String>,
    #[arg(short, long, value_name="out")]
    out_path: Option<String>,
}

fn main() -> Result<()> {
    let myargs = Args::parse();

    let mut buf: Vec<u8> = Vec::new();
    let mut stdin = stdin().lock();
    if atty::is(Stream::Stdin) {
        if myargs.in_path.is_none() {
            return Result::Err(anyhow!("expect src file path"));
        }
        let mut file =  std::fs::File::open(myargs.in_path.as_ref().unwrap())?;
        file.read_to_end(&mut buf)?;
    } else {
        let _ = stdin.read_to_end(&mut buf)?;
    }

    let mut dst = vec![];
    let buf = std::str::from_utf8(&buf)?;
    //println!("{:?}", buf);
    let mut tmp: u8 = 0;
    let mut is_front = true;
    buf.chars().into_iter().
        for_each(|c| {
            let d = c.to_digit(16);
            if d.is_some() {
                if is_front {
                    tmp = (d.unwrap() << 4).min(255) as u8;
                    is_front = false;
                } else {
                    tmp += d.unwrap() as u8;
                    dst.push(tmp);
                    is_front = true;
                    tmp = 0;
                }
            }
        });
    if myargs.out_path.is_none() {
        let _ = stdout().write(&dst);
    } else {
        let out_path = myargs.out_path.unwrap();
        let mut out_file = File::create(&out_path).unwrap();
        let _ = out_file.write(&dst).unwrap();
    }

    Ok(())
}
