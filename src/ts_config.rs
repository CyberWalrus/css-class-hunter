use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader};

#[derive(Deserialize, Debug, Default)]
#[allow(dead_code)]
#[allow(non_snake_case)]
struct Config {
    compilerOptions: Option<CompilerOptions>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
#[allow(non_snake_case)]
struct CompilerOptions {
    target: Option<String>,
    useDefineForClassFields: Option<bool>,
    lib: Option<Vec<String>>,
    allowJs: Option<bool>,
    skipLibCheck: Option<bool>,
    esModuleInterop: Option<bool>,
    allowSyntheticDefaultImports: Option<bool>,
    strict: Option<bool>,
    forceConsistentCasingInFileNames: Option<bool>,
    module: Option<String>,
    moduleResolution: Option<String>,
    resolveJsonModule: Option<bool>,
    noEmit: Option<bool>,
    downlevelIteration: Option<bool>,
    sourceMap: Option<bool>,
    noImplicitAny: Option<bool>,
    jsx: Option<String>,
    baseUrl: Option<String>,
    typeRoots: Option<Vec<String>>,
    rootDirs: Option<Vec<String>>,
    #[serde(default)]
    paths: HashMap<String, Vec<String>>,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        CompilerOptions {
            target: None,
            useDefineForClassFields: None,
            lib: None,
            allowJs: None,
            skipLibCheck: None,
            esModuleInterop: None,
            allowSyntheticDefaultImports: None,
            strict: None,
            forceConsistentCasingInFileNames: None,
            module: None,
            moduleResolution: None,
            resolveJsonModule: None,
            noEmit: None,
            downlevelIteration: None,
            sourceMap: None,
            noImplicitAny: None,
            jsx: None,
            baseUrl: None,
            typeRoots: None,
            rootDirs: None,
            paths: HashMap::new(),
        }
    }
}

pub fn get_paths_ts_config(path: &String) -> io::Result<HashMap<String, Vec<String>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let config: Config = serde_json::from_reader(reader)?;

    // Вернуть paths или пустой HashMap, если compilerOptions или paths отсутствуют
    Ok(config.compilerOptions.unwrap_or_default().paths)
}
