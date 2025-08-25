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
  // pub(super) fn run(options: &Options) -> Result<()> {
  //   let client = options.dogecoin_rpc_client_for_wallet_command(true)?;
  //   // keep your mnemonic/seed generation & persistence in ord data dir
  //   // … (existing seed file write is fine)
  //   let _ = client.call::<serde_json::Value>("getwalletinfo", &[])?; // sanity
  //   println!("ord wallet ready (legacy Dogecoin wallet in Core).");
  //   Ok(())
  // }

  pub(crate) fn run(self, options: Options) -> SubcommandResult {
    let mut entropy = [0; 16];
    rand::thread_rng().fill_bytes(&mut entropy);

    let mnemonic = Mnemonic::from_entropy(&entropy)?;

    initialize_wallet(&options, mnemonic.to_seed(self.passphrase.clone()))?;

    Ok(Box::new(Output {
      mnemonic,
      passphrase: Some(self.passphrase),
    }))
  }
}
