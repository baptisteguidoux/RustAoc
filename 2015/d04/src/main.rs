use md5;

fn main() {
    let input = "yzbqklnj";
    let mut num = 0;

    let mut to_hash = format!("{}{}", input, num.to_string());
    let mut hash = md5::compute(to_hash);
    let mut res_str = format!("{:x}", hash);
    
    while ! res_str.starts_with("000000") {
	num += 1;
	to_hash = format!("{}{}", input, num.to_string());
	hash = md5::compute(to_hash);
	res_str = format!("{:x}", hash);	
    }

    println!("{}", num);
}
