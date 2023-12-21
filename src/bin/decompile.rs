use clap::Parser;
use lowlevel0::parser::Code;
use lowlevel0::pass::const_pass::ConstPass;
use lowlevel0::pass::live_variable_analysis::LiveVariableAnalysisPass;
use lowlevel0::pass::merge_iop_pass::MergeIOPPass;
use lowlevel0::pass::Pass;
use lowlevel0::structures::StructuredInstruction;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(about = "Decompile a ZKR file", long_about = None)]
struct Args {
    // Filename of the ZKR file to be unzipped
    #[arg(short, long, required = true)]
    file: String,

    // Output file, default to [filename].ll0
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();

    let f = File::open(args.file.clone()).unwrap();
    let mut buf_reader = BufReader::new(f);

    let mut u8vec: Vec<u8> = Vec::new();
    buf_reader.read_to_end(&mut u8vec).unwrap();

    let u32vec: Vec<u32> = Vec::from(bytemuck::cast_slice(u8vec.as_slice()));
    let mut code = Code::try_from(u32vec.as_slice()).unwrap();

    let mut const_pass = ConstPass {};
    const_pass.pass(&mut code).unwrap();

    let mut merge_iop_pass = MergeIOPPass {};
    merge_iop_pass.pass(&mut code).unwrap();

    let mut live_variable_analysis = LiveVariableAnalysisPass {};
    live_variable_analysis.pass(&mut code).unwrap();

    let out_name = if args.output.is_some() {
        args.output.unwrap()
    } else {
        let tmp = String::from(Path::new(&args.file).file_name().unwrap().to_str().unwrap());
        if tmp.ends_with(".zkr") {
            String::from(&tmp.as_str()[0..tmp.len() - 4]) + ".ll0"
        } else {
            tmp + ".ll0"
        }
    };

    let ff = File::create(out_name).unwrap();
    let mut buf_writer = BufWriter::new(ff);

    for (insn, line_no) in code.0.iter() {
        match insn {
            StructuredInstruction::__DELETE__ => {}
            _ => {
                buf_writer
                    .write_fmt(format_args!("{}: {}\n", line_no, insn))
                    .unwrap();
            }
        }
    }
}
