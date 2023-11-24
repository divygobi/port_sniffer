
//We are using this to read in arguments from the command line, as you can see in line 37
use std::env;

//This thing lets us do net stuff, have an IpAddr that has the properties of an ip address
use std::net::IpAddr;
//I think this is some shit to convert a string to the u16 thread, or the Ip
use std::str::FromStr;


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
                if let Ok(ip_addr) =  IpAddr::from_str(&first_arg){
                    return Ok(Arguments { flag: String::from(""), threads: (4), ip_addr});
                }
                else if first_arg.contains("-h") || first_arg.contains("-help"){
                    println!("Usage: -j to select how many threads you want
                    \r\n -h or -help to show this help message");
                    return Err("Help");
                }
                else {
                    return Err("Your argument is not valid man, try -h, or -help for help")
                }
            }
            4 => {
                let first_arg = args[1].clone();
                let second_arg = args[2].clone();
                let third_arg = args[3].clone();
                
                if first_arg.contains("-j"){
                    let flag = first_arg.clone();
                    let ip_addr = match IpAddr::from_str(&third_arg) {
                        Ok(s) => s,
                        Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv6"),
                    };
                    let threads = match second_arg.parse::<u16>() {
                        Ok(s) => s,
                        Err(_) => return Err("failed to parse thread number"),
                    };
                    return Ok(Arguments {
                        flag,
                        threads,
                        ip_addr,
                    });
                }
                else{
                    return Err("Your arguments are not valid man, try -h, or -help for help")
                }
            }
            _ => {
                return Err("This has way too many argments or you are putting some nonsense in, try calling this with -h, or -help")
            }

        }
    }
}
fn main() {
    //collects all our arguments into a vector of strings
    //this variable is stored on the heap, so it can grow and shrink at runtime, this is a property of a vector
    let args: Vec<String> = env::args().collect();

    //All of these are all in the heap given that they are STRINGS, Strings are actually just collections of u8s,  they are just a collection(a group of stuff that grow) of numbers in reality
    let program_name = args[0].clone();  
    let flag = args[1].clone();
    let threads = args[2].clone();
    let ip_addr = args[3].clone();
}
