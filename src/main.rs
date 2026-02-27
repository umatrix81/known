use std::env::args;
use std::env::home_dir;
use std::fs::File;
use std::io::Read;
// use std::net::Ipv4Addr;
use std::path::PathBuf;

// fn check_ip_format(ip: &str) -> bool {
//     match ip.parse::<Ipv4Addr>() {
//         Ok(_addr) => true,
//         Err(_) => false,
//     }
// }

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <ip address to remove>", args[0]);
        std::process::exit(0);
    }
    // if !check_ip_format(&args[1]) {
    //     eprintln!("Invalid IP address format. Please provide a valid IPv4 address.");
    //     std::process::exit(0);
    // }
    let host_to_remove = &args[1];

    let homedir = home_dir().expect("Failed to get home directory");
    let known_hosts_path: PathBuf = homedir.join(".ssh\\known_hosts");
    println!("Modifying file: {:?}", known_hosts_path);
    let mut file = File::open(&known_hosts_path).expect("Failed to open known_hosts file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read known_hosts file");

    let occurrences = contents
        .lines()
        .filter(|line| line.contains(host_to_remove))
        .count();
    if occurrences > 0 {
        println!(
            "Found {} occurrence(s) of {:?} in known_hosts.",
            occurrences, host_to_remove
        );
        let filtered: String = contents
            .lines()
            .filter(|line| !line.contains(host_to_remove))
            .map(|line| format!("{}\n", line))
            .collect();

        std::fs::write(&known_hosts_path, filtered)
            .expect("Failed to write updated known_hosts file");
        println!(
            "Removed all occurrences of {:?} from known_hosts.",
            host_to_remove
        );
    } else {
        println!(
            "No occurrences of {:?} found in known_hosts.",
            host_to_remove
        );
    }
}
