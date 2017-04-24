use clap::{Arg, ArgMatches, App, SubCommand, AppSettings};

pub fn create_arg_parser<'a, 'b>() -> App<'a, 'b> {
    App::new("A rust based clone of the all time classic xxd tool")
        .version("0.1.0")
        .author("Nicola Coretti <nicola.coretti@gmail.com>")
        .about("make a hexdump or the reverse")
        .global_settings(&[AppSettings::ColoredHelp])
        .arg(Arg::with_name("infile")
                 .required(false)
                 .global(true)
                 .index(1)
                 .help("Input file which shall be used (default: stdin)"))
        .arg(Arg::with_name("outfile")
                 .required(false)
                 .global(true)
                 .index(2)
                 .help("File to which the output will be written (default: stdout)"))
        .arg(Arg::with_name("seek")
                 .short("s")
                 .long("seek")
                 .required(false)
                 .takes_value(true)
                 .global(true)
                 .help("Offset in the file where to start reading"))
        .arg(Arg::with_name("length")
                 .short("l")
                 .long("length")
                 .required(false)
                 .takes_value(true)
                 .global(true)
                 .help("Amount of bytes which shall be read"))
        .subcommand(SubCommand::with_name("dump")
                        .about("Dumps an input file in the appropriate output format")
                        .arg(Arg::with_name("plain_hexdump")
                                 .short("p")
                                 .long("plain-hexdump")
                                 .required(false)
                                 .help("output in postscript plain hexdump style."))
                        .arg(Arg::with_name("format")
                                 .short("f")
                                 .long("format")
                                 .required(false)
                                 .takes_value(true)
                                 .possible_value("Hex")
                                 .possible_value("hex")
                                 .possible_value("bin")
                                 .possible_value("oct")
                                 .possible_value("dec")
                                 .help("Specifies the output format for the value (default: hex)"))
                        .arg(Arg::with_name("group-size")
                                 .short("g")
                                 .long("group-size")
                                 .required(false)
                                 .takes_value(true)
                                 .help("Separate  the output of every <bytes> bytes (two hex characters or eight bit-digits each) by a whitespace."))
                        .arg(Arg::with_name("columns")
                                 .short("c")
                                 .long("columns")
                                 .required(false)
                                 .takes_value(true)
                                 .help("Specifies the amount of output columns")))
        .subcommand(SubCommand::with_name("convert")
                        .about("Converts input data to a file (e.g. hexstream -> binary file")
                        .arg(Arg::with_name("format")
                                 .short("f")
                                 .long("format")
                                 .required(false)
                                 .takes_value(true)
                                 .possible_value("binary")
                                 .possible_value("c-array")
                                 .possible_value("oct")
                                 .help("Specifies the output format (default: hex)")))
        .subcommand(SubCommand::with_name("generate")
                        .about("Generates a source file containing the specified file as array")
                        .arg(Arg::with_name("template")
                                 .short("t")
                                 .long("template")
                                 .required(false)
                                 .takes_value(true)
                                 .possible_value("c")
                                 .possible_value("cpp")
                                 .possible_value("rs")
                                 .help("Specifies a template file which shall be used for \
                                        generation"))
                        .arg(Arg::with_name("format")
                                 .short("f")
                                 .long("format")
                                 .required(false)
                                 .takes_value(true)
                                 .possible_value("hex")
                                 .possible_value("oct")
                                 .possible_value("dec")
                                 .possible_value("bin")
                                 .help("Specifies the output format (default: hex)"))
                        .arg(Arg::with_name("Separator")
                                 .long("separator")
                                 .required(false)
                                 .takes_value(true)
                                 .help("Specifies the the separator between single values \
                                        (default: ',')")))
}
