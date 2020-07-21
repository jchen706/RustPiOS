mod parsers;

use serial;
use structopt;
use structopt_derive::StructOpt;
use xmodem::{Xmodem, Progress};

use std::path::PathBuf;
use std::time::Duration;

use structopt::StructOpt;
use serial::core::{CharSize, BaudRate, StopBits, FlowControl, SerialDevice, SerialPortSettings};

use parsers::{parse_width, parse_stop_bits, parse_flow_control, parse_baud_rate};

#[derive(StructOpt, Debug)]
#[structopt(about = "Write to TTY using the XMODEM protocol by default.")]
struct Opt {
    #[structopt(short = "i", help = "Input file (defaults to stdin if not set)", parse(from_os_str))]
    input: Option<PathBuf>,

    #[structopt(short = "b", long = "baud", parse(try_from_str = "parse_baud_rate"),
                help = "Set baud rate", default_value = "115200")]
    baud_rate: BaudRate,

    #[structopt(short = "t", long = "timeout", parse(try_from_str),
                help = "Set timeout in seconds", default_value = "10")]
    timeout: u64,

    #[structopt(short = "w", long = "width", parse(try_from_str = "parse_width"),
                help = "Set data character width in bits", default_value = "8")]
    char_width: CharSize,

    #[structopt(help = "Path to TTY device", parse(from_os_str))]
    tty_path: PathBuf,

    #[structopt(short = "f", long = "flow-control", parse(try_from_str = "parse_flow_control"),
                help = "Enable flow control ('hardware' or 'software')", default_value = "none")]
    flow_control: FlowControl,

    #[structopt(short = "s", long = "stop-bits", parse(try_from_str = "parse_stop_bits"),
                help = "Set number of stop bits", default_value = "1")]
    stop_bits: StopBits,

    #[structopt(short = "r", long = "raw", help = "Disable XMODEM")]
    raw: bool,
}
fn progress_fn(progress: Progress) {
    println!("Progress: {:?}", progress);
}

fn main() {
    use std::fs::File;
    use std::io::{self, BufReader};

    let opt = Opt::from_args();
    let mut port = serial::open(&opt.tty_path).expect("path points to invalid TTY");

    // FIXME: Implement the `ttywrite` utility.
    let mut settings = port.read_settings().unwrap();
    settings.set_stop_bits(opt.stop_bits);
    settings.set_flow_control(opt.flow_control);
    settings.set_char_size(opt.char_width);
    settings.set_baud_rate(opt.baud_rate).expect("Baud rate to be set");


    port.write_settings(&settings).expect("Write Settings");


    port.set_timeout(Duration::new(opt.timeout, 0)).expect("set port timeout ");


    // let stdinorinput:Box = match opt.input {
    //     Some(path) => {
    //         let f = File::open(path)?;
    //         let mut reader = BufReader::new(f);
    //         Box::new(reader);
    //     }
    //     None => {
    //         Box::new(io:stdin());
    //     }
    // }
    if opt.raw {
        match opt.input {
            Some(path) => {
                let f = File::open(path).unwrap();
                let mut reader = BufReader::new(f);
                io::copy(&mut reader, &mut port ).expect("io copy with path");
            }
            None => {
                let mut reader = io::stdin();
                io::copy(&mut reader, &mut port).expect("io copy with no path");
            }

        }

    } else {
        match opt.input {
            Some(path) => {
                let f = File::open(path).unwrap();
                let mut reader = BufReader::new(f);
                Xmodem::transmit_with_progress(reader, &mut port, progress_fn).expect("Xmodem transmisiion with a path");
            }
            None => {
                let mut reader = io::stdin();
                Xmodem::transmit_with_progress(reader, &mut port, progress_fn).expect("Xmodem with None opt.input");
            }

        }


    }







}
