use crate::bqn::{self, BQNResult};
use tower_lsp::lsp_types::*;

pub fn get_diagnostics(text: &str) -> Vec<Diagnostic> {
    match bqn::compile(text) {
        Ok(BQNResult::Error { span, error }) => span
            .chunks(2)
            .map(|r| {
                let span_char_count = if r.len() == 1 {
                    1
                } else {
                    (r[1] - r[0]) as usize + 1
                };

                let line = text
                    .chars()
                    .take(r[0] as usize)
                    .filter(|&c| c == '\n')
                    .collect::<String>()
                    .len() as u32;

                let col_start = text
                    .chars()
                    .take(r[0] as usize + span_char_count)
                    .collect::<String>()
                    .chars()
                    .rev()
                    .take_while(|&c| c != '\n')
                    .map(|c| c.len_utf16())
                    .sum::<usize>() as u32;

                let len = text
                    .chars()
                    .skip(r[0] as usize)
                    .take(span_char_count)
                    .map(|c| c.len_utf16())
                    .sum::<usize>() as u32;

                let from = col_start - len;

                Diagnostic::new_simple(
                    Range::new(Position::new(line, from), Position::new(line, from + len)),
                    error.clone(),
                )
            })
            .collect::<Vec<_>>(),
        Ok(_) => vec![],
        Err(e) => {
            eprintln!("{}", e);
            vec![]
        }
    }
}
