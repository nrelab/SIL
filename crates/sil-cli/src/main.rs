#![warn(clippy::pedantic)]

mod commands;
mod formatter;

use clap::Parser;
use commands::build_risk_input;
use formatter::format_decision;
use sil_policy::evaluate;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();

    let normalized = sil_normalizer::normalize_input(&args.input);
    let confusable_flags = sil_confusable::detect_confusables(&args.input);
    let semantic_score = sil_semantic::semantic_similarity(&args.input, "reference");

    let risk = build_risk_input(&confusable_flags, semantic_score);

    let decision = evaluate(risk, &args.input);

    println!("INPUT      : {}", args.input);
    println!("NORMALIZED : {}", normalized);
    println!("FLAGS      : {:?}", confusable_flags);
    println!("DECISION   : {}", format_decision(&decision));
}
