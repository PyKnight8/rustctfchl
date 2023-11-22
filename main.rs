mod misc;
use std::io;
use misc::print_flag;

fn main() {
    let _validpass:&str = "mypasswd";
    let mut _pass = String::new();

    println!("Enter the passphrase:");
    io::stdin().read_line(&mut _pass).expect("Failed to read line");

    _pass = _pass.trim().to_string();
    
    if _validpass==_pass{
        print_flag()
    }
    else{
        println!("You are not worthy!");
    }
    //println!("1:{} 2:{}",_validpass,_pass);
}
