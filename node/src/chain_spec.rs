use hex_literal::hex;
use proofid_runtime_constants::currency::{DECIMALS};
use proofid_node_runtime::{
    AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
    SudoConfig, SystemConfig, WASM_BINARY,
};
use sc_service::{ChainType, Properties};
use sc_telemetry::TelemetryEndpoints;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::crypto::UncheckedInto;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use proofid_common_primitives::{Signature, AccountId,  Balance};

// Constant values used within the runtime.
// use proofid_runtime_constants::{currency::*};

// The URL for the telemetry server.
const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: root_key,
		},
		transaction_payment: Default::default(),
		// treasury: Default::default(),
	}
}

// public proofid network
pub fn public_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or("ProofId testnet was not available")?;
	let boot_nodes = vec![];

	let mut properties = Properties::new();
	properties.insert("ss58Format".into(), 42.into());
	properties.insert("tokenDecimals".into(), DECIMALS.into());
	properties.insert("tokenSymbol".into(), "PID".into());

	Ok(ChainSpec::from_genesis(
		"ProofId Testnet",
		"proofid_testnet",
		ChainType::Live,
		move || public_testnet_genesis(wasm_binary),
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("ProofId testnet telemetry url is valid"),
		),
		Some("proofid_testnet"),
		Some(properties),
		Default::default(),
	))
}

fn public_testnet_genesis(wasm_binary: &[u8]) -> GenesisConfig {
	pub const SUPPLY: Balance = 1500000000_000000; // 300,000,000.000000 * 5 accounts.

	// subkey inspect "$SECRET"
	// for i in 1 2 3 4 5; do for j in genesis; do subkey inspect --scheme sr25519 "$SECRET//$i"; done; done
	let endowed_accounts: Vec<AccountId> = vec![
		// 5GmnH3VwrA6F4ML62f9ppdJbVAcKpKXAZHxFo93yfpCDAfcK
		hex!["d044e014be3145e09863af62e8a8ef0aa9f0051cb7ceeaebe761badb2773a532"].into(),
		// 5E5CjZhF8iEY2YwAZPnoGBq5bzW3LcZ7WwBnTDUds1bEjBR6
		hex!["58d9ad62396e73018d42812f2d19d17dea37c4051bdd5b9117885388cc90f444"].into(),
		// 5FTZ82cux5imcFUEjbFVdY77CFyqW2eB18FsQzPPXLd69gXE
		hex!["9621f07f9388294bb3cb66c45ef5e7ba43d92ab602f9ee1c579222c28d91d651"].into(),
		// 5Eegbc9LseSLKoD8kVeLKQ8yhLLsLXv9i27iDwdrPxFrVgN8
		hex!["726291f609915e542ab022f0fdd61cb87cbd99990d0e3697a6c59dded9568a4e"].into(),
		// 5F1wFYipj2ack2oWwCB8RLnR6iTaz8QRF8UZr3PKfacr9754
		hex!["82980dceaed6127d0113ca75fad8bd925c48583e467d33a6a17aedb1844c6934"].into(),
	];

	// for i in 1 2; do for j in grandpa; do subkey inspect --scheme ed25519 "$SECRET//$i//$j"; done; done
	// for i in 1 2; do for j in aura; do subkey inspect --scheme sr25519 "$SECRET//$i//$j"; done; done
	let initial_authorities: Vec<(AuraId, GrandpaId)> = vec![
		(
			// 5GeaCA9nZoJ8VfKsm77PHxCbbc1ZAQ7hriB3KRP4ZNaQLYvZ //AuraId
			hex!["cac57a74bc889dac0d69f76f13da7c04847be8b6e46c5136e15d444e93a39d5d"]
				.unchecked_into(),
			// 5DeQ9mVEwA2YUKWSXxbDPoZYJUAxQQK3QGe9uyWkcFtvdo2B //GrandpaId
			hex!["45eef9e9dc590b93330895a63e739e42e18d7bf9b6c3782b017a857e7b960ddf"]
				.unchecked_into(),
		),
		(
			// 5CArsNfhDtmmrPPFEFgZnBr9cnkzZHNrwii5bprRVKfSbrvY //AuraId
			hex!["04b1cf61841bff5c98b23d2027eb8fee8bbd231d7adba5ac51e8891d8d7d662d"]
				.unchecked_into(),
			// 5FZ53rZmBLPeoPhho4Js5XRgzHFnfEJsww3Pjqa57wrVzGwa //GrandpaId
			hex!["9a56e9cacf7c47334e8abf3f6044f748526f1b8670d43eda0341ee1987453d5b"]
				.unchecked_into(),
		),
	];

	let num_endowed_accounts = endowed_accounts.len() as u128;
	let endowment: Balance = (SUPPLY / num_endowed_accounts) as Balance;

	GenesisConfig {
		system: SystemConfig { code: wasm_binary.to_vec() },
		balances: BalancesConfig {
			balances: endowed_accounts.iter().map(|k: &AccountId| (k.clone(), endowment)).collect(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		sudo: SudoConfig { key: endowed_accounts[0].clone() },

		transaction_payment: Default::default(),

		// treasury: Default::default(),
	}
}
