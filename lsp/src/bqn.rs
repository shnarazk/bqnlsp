use cbqn::{eval, BQNType, BQNValue, BQN};
use once_cell::sync::OnceCell;
use regex::Regex;

#[derive(Debug)]
pub enum BQNResult {
    Error { span: Vec<u32>, error: String },
    Compiled(CompilerResult),
    EmptyProgram,
}

static GLYPHS_SRC: &str = include_str!(concat!(env!("BQN_PATH"), "src/glyphs.bqn"));
static COMPILER_SRC: &str = include_str!(concat!(env!("BQN_PATH"), "src/c.bqn"));
static COMPILER: OnceCell<(BQNValue, BQNValue)> = OnceCell::new();

#[allow(unused)]
#[derive(Debug)]
pub struct CompilerResult {
    bytecode: Vec<f64>,
    constants: Vec<BQNValue>,
    blocks: Vec<Vec<BQNValue>>,
    bodies: Vec<(f64, f64, Vec<f64>, Vec<f64>)>,
    locs: Vec<Vec<f64>>,
    tokens: (Vec<f64>, Vec<f64>, Vec<BQNValue>, Vec<f64>, Vec<f64>),
}

pub fn compile(code: &str) -> Result<BQNResult, cbqn::Error> {
    if code.is_empty() || code.chars().all(char::is_whitespace) {
        return Ok(BQNResult::EmptyProgram);
    }

    let (compiler, prims) = COMPILER.get_or_init(|| {
        let glyphs = eval(GLYPHS_SRC).unwrap();
        let glyph_strs = glyphs
            .to_bqnvalue_vec()
            .unwrap()
            .into_iter()
            .map(|v| format!(r#""{}""#, v.to_string().unwrap()))
            .collect::<Vec<String>>()
            .join("‿");
        let compiler_src = COMPILER_SRC.replace("•args", &glyph_strs);
        let compiler = eval(&compiler_src).unwrap();
        let compiler = BQN!("{𝕏⎊{𝕊: •CurrentError@}}", compiler).unwrap();
        let prims_system = BQN!(r#"{(∾•BQN∘⋈¨¨𝕩)‿(""‿"."‿""⊸•BQN¨'•'⊸∾¨)}"#, glyphs).unwrap();

        (compiler, prims_system)
    });
    let out = compiler.call2(prims, &BQNValue::from(code))?;
    let res = out.to_bqnvalue_vec()?;

    let invalid_program = res.len() == 2 || res[0].bqn_type() == BQNType::Character;
    if invalid_program {
        let span = match res[0].bqn_type() {
            BQNType::Number => {
                let v = res[0].to_f64()?;
                vec![v, v]
            }
            BQNType::Character => {
                let error = out.to_string()?;
                let words = error.split(' ').collect::<Vec<_>>();
                let mut span = vec![0, 0];
                if let Some(w) = words.iter().next_back() {
                    if w.starts_with("•") {
                        if let Some(new_span) = find_span(w, code) {
                            span = new_span;
                        }
                    }
                }
                return Ok(BQNResult::Error { span, error });
            }
            _ => res[0].to_f64_vec()?,
        };

        let error = res[1].to_string()?;
        Ok(BQNResult::Error {
            span: span.into_iter().map(|v| v as u32).collect(),
            error,
        })
    } else {
        let bytecode = res[0].to_f64_vec()?;
        let constants = res[1].to_bqnvalue_vec()?;

        let blocks = res[2]
            .to_bqnvalue_vec()?
            .into_iter()
            .map(|block| block.to_bqnvalue_vec().unwrap())
            .collect::<Vec<_>>();

        let bodies = res[3]
            .to_bqnvalue_vec()?
            .into_iter()
            .map(|v| {
                let b = v.to_bqnvalue_vec().unwrap();
                (
                    b[0].to_f64().unwrap(),
                    b[1].to_f64().unwrap(),
                    b[2].to_f64_vec().unwrap(),
                    b[3].to_f64_vec().unwrap(),
                )
            })
            .collect::<Vec<_>>();

        let locs = res[4]
            .to_bqnvalue_vec()?
            .into_iter()
            .map(|v| v.to_f64_vec().unwrap())
            .collect::<Vec<_>>();

        let tokens = res[5].to_bqnvalue_vec()?;
        let tokens = (
            tokens[0].to_f64_vec()?,
            tokens[1].to_f64_vec()?,
            tokens[2].to_bqnvalue_vec()?,
            tokens[3].to_f64_vec()?,
            tokens[4].to_f64_vec()?,
        );

        Ok(BQNResult::Compiled(CompilerResult {
            bytecode,
            constants,
            blocks,
            bodies,
            locs,
            tokens,
        }))
    }
}

fn find_span(needle: &str, haystack: &str) -> Option<Vec<u32>> {
    let re = Regex::new(&format!("(?i){}\\b", needle)).unwrap();
    let hit = re.find(haystack)?;

    let start = haystack.bytes().take(hit.start()).collect::<Vec<u8>>();
    let start = String::from_utf8(start).ok()?.chars().count();
    let end = start + needle.trim().chars().count() - 1;

    Some(vec![start as u32, end as u32])
}
