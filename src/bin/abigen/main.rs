use ethers::{contract::Abigen, utils::Solc};
use std::fs;
use convert_case::{Case, Casing};

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    args.next().unwrap(); // skip program name

    let dir = "./abi/";

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(name) = entry.file_name().to_str() {
            let contract_name = name.trim_end_matches(".json");
            let mut binding: String = "./src/bindings/".to_owned();

            let contract_name_snake: String = contract_name.to_case(Case::Snake);
            binding.push_str(&contract_name_snake);
            binding.push_str(".rs");
            dbg!(&path, &contract_name, &binding);

            let abi = fs::read_to_string(path).expect("Something went wrong reading the file");
            let bindings = Abigen::new(&contract_name, abi)?.generate()?;

            // print to stdout if no output arg is given
            bindings.write_to_file(&binding)?;
        }
    }

    Ok(())
}
