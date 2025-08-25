use super::*;

#[derive(Serialize)]
struct Output {
  mnemonic: Mnemonic,
  passphrase: Option<String>,
}

#[derive(Debug, Parser)]
pub(crate) struct Create {
  #[clap(
    long,
    default_value = "",
    help = "Use <PASSPHRASE> to derive wallet seed."
  )]
  pub(crate) passphrase: String,
}

impl Create {
  

  pub(crate) fn run(self, options: Options) -> SubcommandResult {
    let mut entropy = [0; 16];
    rand::thread_rng().fill_bytes(&mut entropy);

    let mnemonic = Mnemonic::from_entropy(&entropy)?;

    log::info!("mnemonic {:#?}",mnemonic);
    initialize_wallet(&options, mnemonic.to_seed(self.passphrase.clone()))?;
    log::info!("wallet initialized");
    Ok(Box::new(Output {
      mnemonic,
      passphrase: Some(self.passphrase),
    }))
  }
}
