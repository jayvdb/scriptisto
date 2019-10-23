use failure::{Error, ResultExt};
use log::debug;
use serde_derive::Deserialize;
use std::cmp::min;
use std::io::{BufRead, BufReader};

#[derive(Deserialize, Debug)]
pub struct BuildSpec {
    pub script_src: String,
    pub build_cmd: String,
    pub target_bin: String,
    #[serde(default)]
    pub replace_shebang_with: String,
    #[serde(default)]
    pub file: Vec<File>,
}

#[derive(Deserialize, Debug)]
pub struct File {
    pub path: String,
    pub content: String,
}

#[derive(Clone, Debug)]
enum ParserState {
    ScriptSource,
    ConfigSource { prefix_len: usize },
}

impl BuildSpec {
    pub fn new(script_body: &[u8]) -> Result<Self, Error> {
        let mut script_src = Vec::new();
        let reader = BufReader::new(script_body);

        use ParserState::*;
        let mut state = ParserState::ScriptSource;
        let mut line_num = 0;

        let mut cfg_src = vec![];

        for line in reader.lines() {
            let mut line = line.context(format!("Cannot parse script line: {}", line_num))?;
            script_src.push(line.clone());
            let old_state = state.clone();
            state = match old_state {
                ScriptSource => {
                    let sb_start = line.find("scriptisto-begin");
                    if let Some(pos) = sb_start {
                        ConfigSource { prefix_len: pos }
                    } else {
                        old_state
                    }
                }
                ConfigSource { prefix_len } => {
                    line.drain(..min(prefix_len, line.len()));
                    if line.starts_with("scriptisto-end") {
                        ScriptSource
                    } else {
                        cfg_src.push(line);
                        old_state
                    }
                }
            };
            line_num += 1;
        }

        let mut build_spec: BuildSpec = toml::from_str(&cfg_src.join("\n"))
            .context(format!("Cannot parse config TOML: \n{:#?}", cfg_src))?;

        let replace_shebang_with = build_spec.replace_shebang_with.clone();
        if !script_src.is_empty() {
            script_src[0] = replace_shebang_with;
        }

        build_spec.file.push(File {
            path: build_spec.script_src.clone(),
            content: script_src.join("\n"),
        });

        debug!("BuildSpec parsed: {:#?}", build_spec);

        Ok(build_spec)
    }
}
