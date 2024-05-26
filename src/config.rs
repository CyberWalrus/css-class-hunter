use serde::Deserialize;
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
    paths: std::collections::HashMap<String, Vec<String>>,
}

pub fn extract_paths_from_file(path: &String) -> io::Result<()> {
    println!("path: {}", path);

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let config: Config = serde_json::from_reader(reader)?;
    println!("{:?}", config.compilerOptions.paths);

    Ok(())
}
