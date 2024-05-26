use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader};

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
#[allow(non_snake_case)]
struct Config {
    compilerOptions: CompilerOptions,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
#[allow(non_snake_case)]
struct CompilerOptions {
    target: String,
    useDefineForClassFields: bool,
    lib: Vec<String>,
    allowJs: bool,
    skipLibCheck: bool,
    esModuleInterop: bool,
    allowSyntheticDefaultImports: bool,
    strict: bool,
    forceConsistentCasingInFileNames: bool,
    module: String,
    moduleResolution: String,
    resolveJsonModule: bool,
    noEmit: bool,
    downlevelIteration: bool,
    sourceMap: bool,
    noImplicitAny: bool,
    jsx: String,
    baseUrl: String,
    typeRoots: Vec<String>,
    rootDirs: Vec<String>,
    pub paths: HashMap<String, Vec<String>>,
}

pub fn get_paths_ts_config(path: &String) -> io::Result<HashMap<String, Vec<String>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let config: Config = serde_json::from_reader(reader)?;

    Ok(config.compilerOptions.paths)
}
