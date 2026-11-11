// A deliberately unsafe file that CodeQL must flag; it is removed in the next commit of this branch.
use std::io::Read;
use std::net::TcpListener;
use std::process::Command;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let (mut stream, _) = listener.accept().unwrap();
    let mut request = String::new();
    stream.read_to_string(&mut request).unwrap();
    let _ = Command::new("sh").arg("-c").arg(&request).status();

    let password = std::env::var("PLANT_PASSWORD").unwrap_or_default();
    println!("password is {}", password);
}
