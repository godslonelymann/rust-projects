use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "csv2parquet", version, about = "converting csv to parquet file")]

struct Args {


    #[arg(long = "in", help = "Input CSV File" )]
    input: PathBuf,

    #[arg(long = "output", help = "Output Parquet File")]
    output: PathBuf,

    #[arg(long = "infer-rows", default_value_t = 1024)] //t is for the typed value
    infer_rows: usize,

}

fn main(){
    let args = Args::parse();

    println!("Input File: {}", args.input.display());
    println!("Output File: {}", args.output.display());
    println!("Infer Rows: {}", args.infer_rows);
}