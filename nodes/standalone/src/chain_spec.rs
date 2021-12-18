// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2021 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

//! KILT chain specification

use pid_primitives::{constants::BLOCKS_PER_YEAR, AccountId, AccountPublic, Balance, BlockNumber};
use proofid_node_runtime::{
	BalancesConfig, CrowdloanContributorsConfig, GenesisConfig, PidLaunchConfig, SessionConfig, SudoConfig,
	SystemConfig, VestingConfig, WASM_BINARY,
};

use hex_literal::hex;

use sc_service::{self, ChainType, Properties};
use sp_consensus_aura::ed25519::AuthorityId as AuraId;
use sp_core::{crypto::UncheckedInto, ed25519, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::IdentifyAccount;

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialised `ChainSpec`. This is a specialisation of the general Substrate
/// ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// The chain specification option. This is expected to come in from the CLI and
/// is little more than one of a number of alternatives which can easily be
/// converted from a string (`--chain=...`) into a `ChainSpec`.
#[derive(Clone, Debug)]
pub enum Alternative {
	/// Whatever the current runtime is, with just Alice as an auth.
	Development,
	/// Whatever the current runtime is, with simple Alice/Bob auths.
	PidTestnet,
	PidDevnet,
	ProofIdStaging,
}

/// Helper function to generate a crypto pair from seed
fn get_from_secret<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(seed, None)
		.unwrap_or_else(|_| panic!("Invalid string '{}'", seed))
		.public()
}

/// Helper function to generate an account ID from seed
fn get_account_id_from_secret<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_secret::<TPublic>(seed)).into_account()
}

/// Helper function to generate an authority key for Aura
fn get_authority_keys_from_secret(seed: &str) -> (AccountId, AuraId, GrandpaId) {
	(
		get_account_id_from_secret::<ed25519::Public>(seed),
		get_from_secret::<AuraId>(seed),
		get_from_secret::<GrandpaId>(seed),
	)
}

/// Build a pair of public keys from a given hex string. This method will panic
/// if the hex string is malformed.
///
/// public_key – the public key formatted as a hex string
fn as_authority_key(public_key: [u8; 32]) -> (AccountId, AuraId, GrandpaId) {
	(
		public_key.into(),
		public_key.unchecked_into(),
		public_key.unchecked_into(),
	)
}

const DEV_AUTH_ALICE: [u8; 32] = hex!("36190a8cfcb7a8f688bf7bc5fbb469a1b64e44a7cf371aa16f82ea0f6436775c");
const DEV_AUTH_BOB: [u8; 32] = hex!("b05835638c0fea2b7b47153cba12e54ac10e156288199ce4408548a5de324d72");
const DEV_AUTH_CHARLIE: [u8; 32] = hex!("8619527b85e20b79f2fb383f6bbeabfdd83f521486610101bfb0e2df66bc755f");
const DEV_FAUCET: [u8; 32] = hex!("406790ed60ef6b9c1eacaf63bad3b26a2004cead7c136f6963bd2b17ed166251");
const TRANSFER_ACCOUNT: [u8; 32] = hex!("0e20827e8cd530ed1a20155e98176292ce343c49af30d2e3d904f8a9b57dd91b");

impl Alternative {
	/// Get an actual chain config from one of the alternatives.
	pub(crate) fn load(self) -> Result<ChainSpec, String> {
		let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm binary not available".to_string())?;

		let mut properties = Properties::new();
		properties.insert("ss58Format".into(), 42.into());
		properties.insert("tokenDecimals".into(), 6.into());
		properties.insert("tokenSymbol".into(), "PID".into());

		Ok(match self {
			Alternative::Development => {
				ChainSpec::from_genesis(
					"Development",
					"development",
					ChainType::Development,
					move || {
						testnet_genesis(
							wasm_binary,
							vec![get_authority_keys_from_secret("//Alice")],
							get_account_id_from_secret::<ed25519::Public>("//Alice"),
							vec![
					// Dev Faucet account
					get_account_id_from_secret::<ed25519::Public>("receive clutch item involve chaos clutch furnace arrest claw isolate okay together"),
					get_account_id_from_secret::<ed25519::Public>("//Alice"),
					get_account_id_from_secret::<ed25519::Public>("//Bob"),
					get_account_id_from_secret::<sr25519::Public>("//Alice"),
					get_account_id_from_secret::<sr25519::Public>("//Bob"),
	],
						)
					},
					vec![],
					None,
					None,
					Some(properties),
					None,
				)
			}
			Alternative::PidTestnet => ChainSpec::from_json_bytes(&include_bytes!("../res/testnet.json")[..])?,
			Alternative::PidDevnet => {
				ChainSpec::from_genesis(
					"ProofId Devnet",
					"proofid_devnet",
					ChainType::Live,
					move || {
						testnet_genesis(
							wasm_binary,
							// Initial Authorities
							vec![
								as_authority_key(DEV_AUTH_ALICE),
								as_authority_key(DEV_AUTH_BOB),
								as_authority_key(DEV_AUTH_CHARLIE),
							],
							DEV_AUTH_ALICE.into(),
							vec![
								DEV_FAUCET.into(),
								DEV_AUTH_ALICE.into(),
								DEV_AUTH_BOB.into(),
								DEV_AUTH_CHARLIE.into(),
							],
						)
					},
					vec![],
					None,
					None,
					Some(properties),
					None,
				)
			}
			Alternative::ProofIdStaging => {
				ChainSpec::from_genesis(
					"ProofId Staging",
					"proofid_staging",
					ChainType::Live,
					move || {
						testnet_genesis(
							wasm_binary,
							// Initial Authorities
							vec![
								as_authority_key(DEV_AUTH_ALICE),
								as_authority_key(DEV_AUTH_BOB),
								as_authority_key(DEV_AUTH_CHARLIE),
							],
							DEV_AUTH_ALICE.into(),
							vec![
								DEV_FAUCET.into(),
								DEV_AUTH_ALICE.into(),
								DEV_AUTH_BOB.into(),
								DEV_AUTH_CHARLIE.into(),
							],
						)
					},
					vec![],
					None,
					None,
					Some(properties),
					None,
				)
			}
		})
	}

	pub(crate) fn from(s: &str) -> Option<Self> {
		match s {
			"dev" => Some(Alternative::Development),
			"testnet" => Some(Alternative::PidTestnet),
			"devnet" => Some(Alternative::PidDevnet),
			"staging" => Some(Alternative::ProofIdStaging),
			_ => None,
		}
	}
}

fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
) -> GenesisConfig {
	type VestingPeriod = BlockNumber;
	type LockingPeriod = BlockNumber;

	// vesting and locks as initially designed
	let airdrop_accounts_json = &include_bytes!("../res/genesis-testing/genesis_accounts.json")[..];
	let airdrop_accounts: Vec<(AccountId, Balance, VestingPeriod, LockingPeriod)> =
		serde_json::from_slice(airdrop_accounts_json).expect("Could not read from genesis_accounts.json");

	GenesisConfig {
		system: SystemConfig {
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			// balances: airdrop_accounts.iter().cloned().map(|(who, total, _, _)| (who, total)).collect(),
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|a| (a, 62_500_000_000_000))
				.chain(airdrop_accounts.iter().cloned().map(|(who, total, _, _)| (who, total)))
				.collect(),
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						proofid_node_runtime::opaque::SessionKeys {
							aura: x.1.clone(),
							grandpa: x.2.clone(),
						},
					)
				})
				.collect::<Vec<_>>(),
		},
		aura: Default::default(),
		grandpa: Default::default(),
		sudo: SudoConfig { key: root_key },
		pid_launch: PidLaunchConfig {
			balance_locks: airdrop_accounts
				.iter()
				.cloned()
				.map(|(who, amount, _, locking_length)| (who, locking_length * BLOCKS_PER_YEAR / 12, amount))
				.collect(),
			vesting: airdrop_accounts
				.iter()
				.cloned()
				.map(|(who, amount, vesting_length, _)| (who, vesting_length * BLOCKS_PER_YEAR / 12, amount))
				.collect(),
			transfer_account: TRANSFER_ACCOUNT.into(),
		},
		crowdloan_contributors: CrowdloanContributorsConfig {
			registrar_account: TRANSFER_ACCOUNT.into(),
		},
		vesting: VestingConfig { vesting: vec![] },
	}
}

pub fn load_spec(id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
	Ok(match Alternative::from(id) {
		Some(spec) => Box::new(spec.load()?),
		None => Box::new(ChainSpec::from_json_file(std::path::PathBuf::from(id))?),
	})
}
