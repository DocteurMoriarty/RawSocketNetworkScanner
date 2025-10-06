///////////////////////////
/// Developed with love ///
///////////////////////////

//Command Line Interface


/*
cargo run -- --src_ip=192.168.25.2
cargo run -- --dst_ip=192.168.1.25
cargo run -- --dest_port=8080
cargo run -- --src_mac=aa:bb:cc:dd:ee:ff
cargo run -- --dst_mac=11:22:33:44:55:66
cargo run -- --l4_protocol=udp
cargo run -- --l4_protocol=tcp
cargo run -- --timeout_ms=2000
cargo run -- --debug_file=./debug.pcap --debug_format=pcap
cargo run -- --debug_file=./debug.json --debug_format=json
cargo run -- --ip_bitfield=0x04 --dry_run
*/


// Struct for params in project

pub mod args {
    use clap::{
        Parser,
        CommandFactory
    };

    use crate::parsing::my_parser::{
        parse_mac, 
        parse_hex
    };

    // Publics pour encapsulation
    impl Args {
        pub fn src_ip(
            &self
        ) -> &Option<String> { 
            &self.src_ip 
        }
        pub fn dst_ip(
            &self
        ) -> &Option<String> { 
            &self.dst_ip 
        }
        pub fn dest_port(
            &self
        ) -> &Option<u16> { 
            &self.dest_port 
        }
        pub fn src_mac(
            &self
        ) -> &Option<[u8; 6]> { 
            &self.src_mac 
        }
        pub fn dst_mac(
            &self
        ) -> &Option<[u8; 6]> { 
            &self.dst_mac 
        }
        pub fn l4_protocol(
            &self
        ) -> &Option<String> { 
            &self.l4_protocol 
        }
        pub fn timeout_ms(
            &self
        ) -> &Option<u64> { 
            &self.timeout_ms 
        }
        pub fn debug_file(
            &self
        ) -> &Option<String> { 
            &self.debug_file 
        }
        pub fn debug_format(
            &self
        ) -> &Option<String> { 
            &self.debug_format 
        }
        pub fn ip_bitfield(
            &self
        ) -> &Option<u8> { 
            &self.ip_bitfield 
        }
        pub fn dry_run(
            &self
        ) -> &Option<bool> { 
            &self.dry_run 
        }
        pub fn count(
            &self
        ) -> &u32 { 
            &self.count 
        }
    }


    #[derive(Parser, Debug)]
    #[command(version, about, long_about = None)]
    pub struct Args {
        // Short and long it's arguments -p --port
        #[arg(
            short, 
            help = "format: --dst_ip=192.168.25.2", 
            long = "src_ip"
        )]
        src_ip: Option<String>,

        #[arg(
            short, 
            help = "format: --dst_ip=192.168.1.25", 
            long = "dst_ip"
        )]
        dst_ip: Option<String>,

        #[arg(
            short = 'p', 
            help = "format: --dest_port=8080", 
            long = "dest_port"
        )]
        dest_port: Option<u16>,

        // Parser personnalised
        #[arg(
            short = 'm' ,
            long = "src_mac", 
            help = "format: --src_mac=aa:bb:cc:dd:ee:ff",
            value_parser = parse_mac
        )]
        src_mac: Option<[u8; 6]>,

        #[arg(
            short = 'c' ,
            long = "dst_mac", 
            help = "format: --src_mac=11:22:33:44:55:66",
            value_parser = parse_mac
        )]
        dst_mac: Option<[u8; 6]>,

        #[arg(
            short, 
            help = "format: --l4_protocol=udp", 
            long = "l4_protocol"
        )]
        l4_protocol: Option<String>,

        #[arg(
            short, 
            long = "timeout_ms", 
            help = "format: --timeout_ms=2000"
        )]
        timeout_ms: Option<u64>,

        #[arg(
            short = 'f',
            long = "debug_file",
            help = "format: --debug_file=./debug.pcap"
        )]
        debug_file: Option<String>,

        #[arg(
            short = 'g',
            long = "debug_format",
            help = "format: --debug_format=json",
            default_value = "json"
        )]
        debug_format: Option<String>,

        #[arg(
            short = 'b',
            long = "ip_bitfield", 
            default_value = "0x00",
            help = "format: --ip_bitfield=0x04",
            value_parser = parse_hex
        )]
        ip_bitfield: Option<u8>,

        #[arg(
            short = 'r',
            long = "dry_run", 
            help = "format: --dry_run",
            action = clap::ArgAction::SetTrue
        )]
        dry_run: Option<bool>,

        //count for iterations in structs
        #[arg(
            long, 
            default_value_t = 1, 
            hide = true
        )]
        count: u32,
    }

    pub fn parse_args() -> Args {
        if std::env::args().len() == 1 {
            let mut cmd = Args::command();
            cmd.print_help().unwrap();
            println!();   
            std::process::exit(0);
        }
        Args::parse()
    }

}




