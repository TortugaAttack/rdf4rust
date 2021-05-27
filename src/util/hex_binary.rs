
pub fn string_to_binary(hex_string: &str) -> Option<Vec<u8>>{
    let mut last =0;
    let mut old=false;
    let mut ret:Vec<u8> = Vec::new();
    for c in hex_string.chars(){
        let mut tmp = map_to_binary(c);
        //check if valid hex
        if tmp==16 {return None;}
        if !old{
            last = (tmp << 4);
            old=true
        }
        else{
            ret.push(last+tmp);
            old=false
        }
    }
    Some(ret)
}

pub fn binary_to_string(binary: &[u8]) -> String{
    let hex_vec: Vec<String> = binary.iter().map(|b| format!("{:02X}", b)).collect();
    hex_vec.join("")

}

fn map_to_binary(c: char)->u8{
    return match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'A' => 10,
        'B' => 11,
        'C' => 12,
        'D' => 13,
        'E' => 14,
        'F' => 15,
        _ => 16,
    }
}