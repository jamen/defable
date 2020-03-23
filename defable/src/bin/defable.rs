use fable_inject::{Inject,InjectTarget};

use clap::{App,Arg};

fn main() -> Result<(), u32> {
    let matches =
        App::new("defable")
        .version("0.1.0")
        .about("Fable mod tool.")
        .author("Jamen Marz <me@jamen.dev>")
        .arg(
            Arg::with_name("exe")
            .long("exe")
            .help("Path to Fable's executable.")
            .conflicts_with_all(&["pid", "find"])
            .required(false)
            .takes_value(true)
        )
        .arg(
            Arg::with_name("pid")
            .long("pid")
            .help("Attach to Fable process by PID.")
            .conflicts_with_all(&["exe", "find"])
            .required(false)
            .takes_value(true)
        )
        .arg(
            Arg::with_name("find")
            .long("find")
            .help("Attempts to find ")
            .conflicts_with_all(&["exe", "pid"])
            .required(false)
            .takes_value(true)
            .default_value("Fable.exe")
        )
        .arg(
            Arg::with_name("dll")
            .long("dll")
            .required(false)
            .help("Path to DLL hack.")
            .takes_value(true)
        )
        .get_matches();

    let method =
        if let Some(value) = matches.value_of("create") { InjectTarget::Create(value) }
        else if let Some(value) = matches.value_of("pid") { InjectTarget::Pid(value) }
        else if let Some(value) = matches.value_of("find") { InjectTarget::Find(value) }
        else { InjectTarget::Find("Fable.exe") };

    let injector = Inject {
        target: method,
        dll: matches.value_of("dll"),
    };

    injector.start().unwrap();

    Ok(())
}