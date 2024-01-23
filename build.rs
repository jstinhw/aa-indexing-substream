use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("Safe", "abi/Safe.json")?
        .generate()?
        .write_to_file("src/abi/safe.rs")?;
    Abigen::new("FunFactory", "abi/FunFactory.json")?
        .generate()?
        .write_to_file("src/abi/fun_factory.rs")?;
    Abigen::new("BiconomyFactoryV2", "abi/BiconomyFactoryV2.json")?
        .generate()?
        .write_to_file("src/abi/biconomy_factory_v2.rs")?;
    Abigen::new("BloctoAccountFactory", "abi/BloctoAccountFactory.json")?
        .generate()?
        .write_to_file("src/abi/blocto_account_factory.rs")?;
    Ok(())
}