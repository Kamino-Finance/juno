use std::collections::HashSet;

/// Example inspired from mvines's [rust-jup-ag](https://github.com/mvines/rust-jup-ag/blob/master/examples/swap.rs)
use itertools::Itertools;
use juno::DecompiledVersionedTx;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
};
use spl_token::{amount_to_ui_amount, ui_amount_to_amount};

const USDH: Pubkey = pubkey!("USDH1SM1ojwWUga67PGrgFWUHibbjqMvuMaDkRJTgkX");
const HBB: Pubkey = pubkey!("HBB111SCo9jkCejsZfz8Ec8nH7T6THF8KEKSnvwT6XK6");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let keypair = read_keypair_file("swap_example.json").unwrap_or_else(|err| {
        println!("------------------------------------------------------------------------------------------------");
        println!("Failed to read `swap_example.json`: {}", err);
        println!();
        println!("An ephemeral keypair will be used instead. For a more realistic example, create a new keypair at");
        println!("that location and fund it with a small amount of USDH.");
        println!("------------------------------------------------------------------------------------------------");
        println!();
        Keypair::new()
    });

    let rpc_client = RpcClient::new_with_commitment(
        "https://api.mainnet-beta.solana.com".into(),
        CommitmentConfig::processed(),
    );

    let usdh_ata =
        spl_associated_token_account::get_associated_token_address(&keypair.pubkey(), &USDH);

    let hbb_ata =
        spl_associated_token_account::get_associated_token_address(&keypair.pubkey(), &HBB);

    println!(
        "Pre-swap USDH balance: {}",
        amount_to_ui_amount(
            rpc_client
                .get_token_account_balance(&usdh_ata)
                .await?
                .amount
                .parse::<u64>()?,
            6
        )
    );

    println!(
        "Pre-swap HBB balance: {}",
        amount_to_ui_amount(
            rpc_client
                .get_token_account_balance(&hbb_ata)
                .await?
                .amount
                .parse::<u64>()?,
            6
        )
    );

    // set Jup base URL
    juno::set_base_url("https://quote-api.jup.ag".to_string())?;

    let quote: juno::SwapRoute = juno::get_quote(
        &USDH,
        &HBB,
        ui_amount_to_amount(1.0, 6),
        false,
        Some(100),
        Some(20),
    )
    .await?;

    let route = quote
        .route_plan
        .iter()
        .map(|plan| plan.swap_info.label.clone())
        .join(", ");
    println!(
        "Quote: {} USDH for {} HBB via {} (worst case with slippage: {}). Impact pct: {}%",
        amount_to_ui_amount(quote.in_amount, 6),
        amount_to_ui_amount(quote.out_amount, 6),
        route,
        amount_to_ui_amount(quote.other_amount_threshold, 6),
        quote.price_impact_pct
    );

    let decompiled_ixs = juno::get_swap_instructions(quote, keypair.pubkey(), &rpc_client).await?;

    println!("Swap ixs received: {decompiled_ixs:#?}");

    // Recompile and simulate
    let DecompiledVersionedTx {
        lookup_tables,
        instructions,
    } = decompiled_ixs;

    let mut distinct_accounts: HashSet<Pubkey> = HashSet::new();
    instructions.iter().for_each(|ix| {
        ix.accounts.iter().for_each(|acc| {
            distinct_accounts.insert(acc.pubkey);
        })
    });
    println!("swap will use {} accounts", distinct_accounts.len());

    let tx = juno::utils::create_tx_with_address_table_lookup(
        &instructions,
        &lookup_tables.unwrap_or_default(),
        rpc_client.get_latest_blockhash().await?,
        &keypair,
    )?;

    let sim = rpc_client.simulate_transaction(&tx).await?;

    println!("Simulation result:\n{sim:#?}");

    Ok(())
}
