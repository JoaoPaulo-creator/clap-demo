use clap::Parser;
mod args;
use args::Args;

fn main() {
    let args = Args::parse();

    let env = match args.env.as_str() {
        "dev" => "dev".to_string(),
        "stage" => "staging".to_string(),
        _ => "uat".to_string(),
    };

    assemble_env(env);
}

fn assemble_env(e: String) {
    println!("Acessando o ambiente: {}", e);
}
