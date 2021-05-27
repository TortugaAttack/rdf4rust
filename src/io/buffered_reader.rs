use std::fs::File;
use std::io::Read;
use std::fmt;
use std::error::Error;


const BUFFER_SIZE: usize = 1024;

pub struct BufferedReader{
    file: File,
    buffer_lines: Vec<String>,
    unresolved_line: String,
    eof: bool
}

impl BufferedReader{

    pub fn new(file_name: &str) -> Result<BufferedReader, IOError>{
        let file = match File::open(&file_name){
            Ok(file) => {file}
            Err(err) => {return Err(IOError::new(err.to_string()))}
        };
        Ok(BufferedReader{
            file,
            buffer_lines: Vec::new(),
            unresolved_line: String::new(),
            eof: false
        })
    }

    ///
    ///
    /// return 0 doesn't mean end of file, but that the line was bigger than the buffer it was read to. For that please use self.eof.
    ///
    ///
    fn read_lines_to_buffer(&mut self) -> Result<usize, IOError>{
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        let mut size_read=BUFFER_SIZE;

        size_read = match self.file.read(&mut buffer) {
            Ok(num) => {num}
            Err(err) => {return Err(IOError::new(err.to_string()))}
        };
        if size_read != 0 {
            let mut complete_buffer: Vec<u8> = Vec::new();

            let mut unresolved: Vec<_> = self.unresolved_line.bytes().collect();
            complete_buffer.append(&mut unresolved);
            let tmp = &buffer[..size_read];
            complete_buffer.append(&mut Vec::from(tmp));
            self.unresolved_line.clear();
            //buffer to string
            let utf8str = String::from_utf8_lossy(&complete_buffer);
            let mut lines: Vec<_> = utf8str.split("\n").collect();
            //check if last byte is new line
            if buffer[size_read-1] != b'\n'{
                self.unresolved_line += &lines.pop().expect("Vector is unexpectedly empty.");
            }
            let mut tmp: Vec<String> = lines.iter().map(|&x| String::from(x)).collect();
            self.buffer_lines.append(&mut tmp);
        }
        else if !self.eof{

            self.eof = true;
            self.buffer_lines.insert(0, String::from(&self.unresolved_line));
            self.unresolved_line.clear();
        }
        return Ok(self.buffer_lines.len());
    }

    pub fn read_line(&mut self) -> Option<String>{

        while self.buffer_lines.is_empty(){
            self.read_lines_to_buffer().expect("");
        }
        if self.eof{
            //EOF
            return None
        }
        Some(self.buffer_lines.remove(0))
    }
}

#[derive(Debug)]
pub struct IOError{
    msg: String
}

impl Error for IOError {}

impl fmt::Display for IOError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.msg)
    }
}

impl IOError {
    pub fn new(msg: String) -> IOError {
        IOError {msg}
    }
}