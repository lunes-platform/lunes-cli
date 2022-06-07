use crate::wallet::WalletNew;
use lunesweb;


pub fn create_wallet(args: WalletNew) {
  let x = lunesweb::wallet::Wallet::new_seed(
    args.words,
    args.nonce,
    args.chain
  );
  println!("seed: {}\nprivate key: {}\naddress: {}\nchain: {}\nnonce: {}", x.seed, x.private_key, x.address, x.chain, x.nonce);
}
