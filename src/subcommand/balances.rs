use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
  pub dunes: BTreeMap<SpacedDune, BTreeMap<OutPoint, u128>>,
}

pub(crate) fn run(options: Options) -> SubcommandResult {
  let index = Index::open(&options)?;

  ensure!(
    index.has_dune_index(),
    "`ord balances` requires index created with `--index-dunes` flag",
  );

  index.update()?;

  Ok(Box::new(Output {
    dunes: index.get_dune_balance_map()?,
  }))
}

// pub(super) fn run(options: &Options) -> Result<()> {
//   let client = options.dogecoin_rpc_client_for_wallet_command(false)?;
//   // getbalance -> number of DOGE (double)
//   let doge: f64 = client.call("getbalance", &[])?;
//   println!("{:.8} DOGE", doge);
//   Ok(())
// }
