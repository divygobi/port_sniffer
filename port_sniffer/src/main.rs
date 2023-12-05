
//We are using this to read in arguments from the command line, as you can see in line 37
use std::{env, thread};
use std::io::{self, Write};
// use std::fmt::Arguments;
//This thing lets us do net stuff, have an IpAddr that has the properties of an ip address
use std::net::IpAddr;
use std::net::TcpStream;
// use std::os::unix::process;
use std::process::exit;
//I think this is some shit to convert a string to the u16 thread, or the Ip
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use local_ip_address::local_ip;

// use std::sync::mpsc;

const MAX: u16 = 65535;

//This is a type/struct, in other languages like java, you would define methods here, but it seems in Rust we only store MEMBERS of a struct here
struct Arguments{
    flag: String,
    threads: u16,
    ip_addr: IpAddr,
}

//This is an imlp block, so it declares and defines the methods of the struct
//This 
impl Arguments{
    fn new(args: &[String]) -> Result<Arguments, &'static str>{
        match args.len(){
            0 | 1 => {
                return Err("You must have more arguments man, this ain't right, try calling this with -h, or -help")
            }
            2 => {
                let first_arg: String = args[1].clone();
                if let Ok(ip_addr) = IpAddr::from_str(&first_arg){
                    return Ok(Arguments { flag: String::from(""), threads: (4), ip_addr});
                }
                else if first_arg.contains("-h") || first_arg.contains("-help"){
                    println!("Usage: -j to select how many threads you want
                    \r\n-h or -help to show this help message");
                    return Err("Help");
                }
                else if first_arg.contains("localhost") {
                    let my_local_ip = local_ip().unwrap();
                    return Ok(Arguments { flag: String::from(""), threads: (4), ip_addr: my_local_ip});
                }
                else {
                    return Err("Your ip is not valid man, try -h,")
                }
            }
            4 => {
                let first_arg = args[1].clone();
                let second_arg = args[2].clone();
                let third_arg = args[3].clone();
                
                if first_arg.contains("-j"){
                    let flag = first_arg.clone();
                    let threads = match second_arg.parse::<u16>() {
                        Ok(s) => s,
                        Err(_) => return Err("failed to parse thread number"),
                    };
                    if third_arg.contains("localhost"){
                        let my_local_ip = local_ip().unwrap();
                        return Ok(Arguments { flag: String::from(""), threads: (4), ip_addr: my_local_ip});
                    }
                    let ip_addr = match IpAddr::from_str(&third_arg) {
                        Ok(s) => s,
                        Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv6"),
                    };
                   
                    return Ok(Arguments {
                        flag,
                        threads,
                        ip_addr,
                    });
                }
                else{
                    return Err("Your arguments are not valid man, try -h,")
                }
            }
            _ => {
                return Err("This has way too many argments or you are putting some nonsense in, try calling this with -h,")
            }
        }
        // return Err("Something went horribly wrong")
    }
}

fn scan(tx: Sender<u16>, start_port: u16, ip_addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((ip_addr, port)) {
            Ok(_) => { 
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {
                print!("")
            }
        }
        if (MAX - port) < num_threads {
            break;
        }
        port += num_threads;
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = match Arguments::new(&args){
        Ok(args) => args,
        Err(e) => {
            if e.contains("Help"){
                exit(0);
            }
            else{
                eprintln!("{} problem parsing the arguments: {}", program, e);
                exit(0);
            }
        }
    };
    println!("{} is the ip", arguments.ip_addr.to_string());
    let num_threads = arguments.threads;
    let addr = arguments.ip_addr;
    let (tx, rx) = channel();
    for i in 0..num_threads{
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}