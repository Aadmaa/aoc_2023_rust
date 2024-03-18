#[macro_use]
extern crate enum_display_derive;

use clap::Parser;
use serde::Serialize;
use std::{fmt::Display, path::PathBuf};

pub mod utils;
mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;

pub fn hello() -> Result<u64, String> {
    let args = Args::parse();

    match args.problem {
        ProblemNumber::D1 => d01::run_d1(args).map(|val| val as u64),
        ProblemNumber::D2 => d02::run_d2(args).map(|val| val as u64),
        ProblemNumber::D3 => d03::run_d3(args).map(|val| val as u64),
        ProblemNumber::D4 => d04::run_d4(args).map(|val| val as u64),
        ProblemNumber::D5 => d05::run_d5(args),
        ProblemNumber::D6 => d06::run_d6(args),
    }
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Specifies a problem number
    #[arg(short, long, value_enum)]
    problem: ProblemNumber,

    /// Problem part (defaults to "a")
    #[arg(short = 't', long, value_enum)]
    part: ProblemPart,

    /// Relative location of the input file
    #[arg(short, long)]
    file: PathBuf,
}

impl Args {
    // Create Args instance for tests
    pub fn new(problem: ProblemNumber, part: ProblemPart, file: &str) -> Self {
        Args {
            problem,
            part,
            file: PathBuf::from(file),
        }
    }
}

#[derive(Display)]
#[derive(
    clap::ValueEnum, Clone, Debug, Serialize
)]
#[serde(rename_all = "kebab-case")]
pub enum ProblemNumber {
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
}

#[derive(Display)]
#[derive(
    clap::ValueEnum, Clone, Debug, Serialize, Default
)]
#[serde(rename_all = "lowercase")]
pub enum ProblemPart {
    #[default] A,
    B,
}
