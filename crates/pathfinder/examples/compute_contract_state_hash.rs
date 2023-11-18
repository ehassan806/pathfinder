use anyhow::Context;
use pathfinder_crypto::{hash::pedersen_hash, Felt};

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1); // Skip the first argument, which is the program name

    let myself = args.next().unwrap_or_else(|| String::from("compute_contract_state_hash"));

    let args = args.map(|x| Felt::from_hex_str(&x).map(Some)).chain(std::iter::repeat_with(|| Ok(None)));

    let mut description = String::with_capacity(1 + 64 + 64 + 64 + 64 + 4);
    description.push('#');

    let res = args
        .enumerate()
        .inspect(|(_, x)| {
            if let Ok(x) = x {
                description.push_str(&format!(" {:x}", x.unwrap_or(Felt::ZERO)));
            }
        })
        .take(4)
        .try_fold(None, |acc, (nth, x)| {
            let nth = nth + 1;
            let next = x.with_context(|| format!("Failed to parse {} parameter", nth))?;
            let next = if nth < 2 {
                next.with_context(|| format!("Missing {} parameter", nth))?
            } else {
                next.unwrap_or(Felt::ZERO)
            };
            Ok(acc.map(|prev| pedersen_hash(prev, next)).or(Some(next)))
        })
        .with_context(|| format!("USAGE: {} class_hash tree_root [nonce [contract_version]]", myself))?
        .expect("there is always iterated over value");

    println!("{}", description);
    println!("{:x}", res);

    Ok(())
}
