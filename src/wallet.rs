use super::*;

#[derive(Copy, Clone)]
pub(crate) struct Wallet {
  _private: (),
}
impl Wallet {
  pub(crate) fn load(options: &Options) -> Result<Self> {
    if let Err(e) = options.dogecoin_rpc_client_for_wallet_command(false) {
      log::warn!("Continuing without strict wallet checks: {}", e);
    }
    Ok(Self { _private: () })
  }
}