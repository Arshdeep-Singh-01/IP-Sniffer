use std::env;
use std::net::IpAddr ;
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;

const MAX: u16 = 65535;

struct Agrs {
    flag: String,
    ip: IpAddr,
    threads: u16,
}

impl Agrs {
    fn new(args: &[String]) -> Result<Agrs, &'static str> {
        if args.len() < 2 {
            // program name and the ipaddress
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }
        let ip_addr = args[1].clone();
        // check if the ip is valid or is it a -h flag 
        if let Ok(ip) = IpAddr::from_str(&ip_addr) {
            if args.len() == 2 {
                return Ok(Agrs { flag: String::from(""), ip, threads: 4 });
            }
            else{
                let flag = args[2].clone();
                if flag == "-t" {
                    let threads = match args[3].parse() {
                        Ok(t) => t,
                        Err(_) => return Err("Failed to parse number of threads"),
                    };

                    return Ok(Agrs { flag, ip, threads });
                } else if flag == "-h" {
                    // return help message about the possible flags and the usage
                    let message: String = String::from("Usage: <function> <ip> <flag> <threads>\n flag can be -t for number of threads or -h for help");
                    println!("{}", message);
                    return Err("help message");
                } else {
                    return Err("invalid flag: should be -t or -h");
                }
            }
        } else {
            // if ip is a -h flag
            if args[1] == "-h" {
                let message: String = String::from("Usage: <function> <ip> <flag> <threads>\n flag can be -t for number of threads or -h for help");
                println!("{}", message);
                return Err("help message");
            } else {
                return Err("invalid ip address");
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match std::net::TcpStream::connect((addr, port)) {
            Ok(_) => {
                println!("{} is open", port);
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }
        if (MAX - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}

fn main() { 
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Agrs::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help"){
                process::exit(0);
            }
            else{
                eprintln!("{} problem parsing arguments: {}", program, err);
                process::exit(1);
            }
        }
    );

    let num_threads = arguments.threads;
    let (tx,rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, arguments.ip, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }
    println!("{:?}", out);

}
