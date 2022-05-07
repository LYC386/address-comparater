use std::{env, process};
struct Addr {
    addr: String,
}

fn compare(input1: &Addr, input2: &Addr) -> bool {
    input1.addr == input2.addr
}

fn main() {
    let mut arg_iter = env::args();
    arg_iter.next();
    let addr1 = match arg_iter.next() {
        Some(arg) => arg,

        None => {
            eprintln!("missing first addr!");
            process::exit(1);
        }
    };

    let addr2 = match arg_iter.next() {
        Some(arg) => arg,

        None => {
            eprintln!("missing second addr!");
            process::exit(1);
        }
    };

    let same = compare(&Addr { addr: addr1 }, &Addr { addr: addr2 });
    if same {
        println!("Addrs are the same!");
        process::exit(0);
    } else {
        println!("Addrs are different!");
        process::exit(0);
    }
}
