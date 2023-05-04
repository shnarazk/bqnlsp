use anyhow::Result;
use std::{
    fs,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

use cbqn::{eval, BQNValue, BQN};

fn usage() -> ! {
    println!("usage");
    std::process::exit(1);
}

fn handle_dir(help_path: PathBuf, output_path: PathBuf) -> Result<()> {
    for entry in fs::read_dir(&help_path)? {
        let name = entry?.file_name();
        if let Ok(name) = name.into_string() {
            if name.ends_with(".md") {
                let repl = BQN!(r#"•ReBQN {repl⇐"strict"}"#)?;
                let f = BufReader::new(fs::File::open(help_path.join(&name))?);
                let mut outf = BufWriter::new(fs::File::create(output_path.join(name))?);
                let mut skipnext = false;
                let mut inside_code_block = false;
                for line in f.lines() {
                    let mut line = line?;
                    if line.contains("View this file with results and syntax highlighting")
                        || skipnext
                    {
                        skipnext = !skipnext;
                        continue;
                    }
                    if line.contains("../doc/") {
                        line = line.replace("../doc/", "https://mlochbaum.github.io/BQN/doc/");
                        line = line.replace(".md", ".html");
                    }
                    if line.contains("../spec/") {
                        line = line.replace("../spec/", "https://mlochbaum.github.io/BQN/spec/");
                        line = line.replace(".md", ".html");
                    }

                    let bqn_out = if line.starts_with("        ") {
                        line = line.replace("        ", "   ");
                        if let Some(i) = line.find('#') {
                            line = line[..i].to_string();
                        }
                        let bqn_code = format!("{}", line);
                        let out = BQN!(
                            repl.clone(),
                            "{•Fmt 𝕎𝕩}⎊{𝕊: (⊑·/(@+10)⊸=)⊸↑ ∾(•CurrentError@)‿(@+10)}",
                            bqn_code
                        )?
                        .to_string()?;

                        if !inside_code_block {
                            inside_code_block = true;
                            line.insert_str(0, "```bqn\n");
                        }

                        let is_assignment_only = (line.contains("←") || line.contains("↩"))
                            && line.chars().nth(3).unwrap().is_alphanumeric();

                        if !is_assignment_only {
                            Some(out)
                        } else {
                            None
                        }
                    } else {
                        if !line.is_empty() && inside_code_block {
                            inside_code_block = false;
                            line.insert_str(0, "```\n");
                        }
                        None
                    };
                    outf.write(line.as_bytes())?;
                    outf.write(b"\n")?;
                    if let Some(bqn_out) = bqn_out {
                        outf.write(bqn_out.as_bytes())?;
                        outf.write(b"\n")?;
                    }
                }
                if inside_code_block {
                    outf.write(b"```\n")?;
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let mut args = std::env::args();
    let bqn_path = match args.nth(1) {
        Some(bqn) => PathBuf::from(bqn),
        None => usage(),
    };

    match bqn_path.try_exists() {
        Ok(true) => (),
        _ => {
            println!("BQN repository not found or access denied.");
            println!("Maybe run `git submodule update --init --recursive`");
            std::process::exit(1);
        }
    }

    let output = match args.nth(0) {
        Some(out) => PathBuf::from(out),
        None => usage(),
    };

    let _ = fs::create_dir(&output);
    handle_dir(bqn_path.join("help"), output).unwrap();
}
