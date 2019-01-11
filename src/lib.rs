#[allow(unused_macros)]
macro_rules! call_with_file_or_stdin {
    ($func: ident, $fname: expr) => {
        match $fname {
            Some(fname) => {
                let file = std::fs::File::open(fname).expect("call_with_file_or_stdin");
                $func(std::io::BufReader::new(file))
            },
            None => {
                let stdin = std::io::stdin();
                let val = $func(stdin.lock());
                val
            },
        }
    };
}

#[allow(unused_macros)]
macro_rules! call_with_file_or_stdout {
    ($func: ident, $fname: expr) => {
        match $fname {
            Some(fname) => {
                let file = std::fs::File::create(fname).expect("call_with_file_or_stdout");
                $func(std::io::BufWriter::new(file))
            },
            None => {
                let stdout = std::io::stdout();
                let val = $func(stdout.lock());
                val
            },
        }
    };
}

#[cfg(test)]
mod test {
    use std::io::{Read, Write};

    fn write_data<W: Write>(mut stream: W) -> usize {
        stream.write(b"Hello, world!\n").unwrap()
    }

    fn read_data<R: Read>(mut stream: R) -> usize {
        let mut buf = [0; 16];
        let len = stream.read(&mut buf).unwrap();
        println!("{}", std::str::from_utf8(&buf).unwrap());
        len
    }

    #[test]
    fn test_out() {
        let path: Option<String> = None;
        let len = call_with_file_or_stdout!(write_data, path);
        println!("len = {}", len);
    }

    #[test]
    fn test_in() {
        let path = Some("src/lib.rs");
        let len = call_with_file_or_stdin!(read_data, path);
        println!("len = {}", len);
    }
}
