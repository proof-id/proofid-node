// --- paritytech ---
use parachain_staking::Config;

// --- proofid-network ---
use crate::*;

parameter_types! {
	/// Minimum round length is 1 hour
	pub const MinBlocksPerRound: BlockNumber = constants::staking::MIN_BLOCKS_PER_ROUND;
	/// Default length of a round/session is 2 hours
	pub const DefaultBlocksPerRound: BlockNumber = constants::staking::DEFAULT_BLOCKS_PER_ROUND;
	/// Unstaked balance can be unlocked after 7 days
	pub const StakeDuration: BlockNumber = constants::staking::STAKE_DURATION;
	/// Collator exit requests are delayed by 4 hours (2 rounds/sessions)
	pub const ExitQueueDelay: u32 = 2;
	/// Minimum 16 collators selected per round, default at genesis and minimum forever after
	pub const MinCollators: u32 = constants::staking::MIN_COLLATORS;
	/// At least 4 candidates which cannot leave the network if there are no other candidates.
	pub const MinRequiredCollators: u32 = 4;
	/// We only allow one delegation per round.
	pub const MaxDelegationsPerRound: u32 = 1;
	/// Maximum 25 delegators per collator at launch, might be increased later
	#[derive(Debug, PartialEq)]
	pub const MaxDelegatorsPerCollator: u32 = 25;
	/// Maximum 1 collator per delegator at launch, will be increased later
	#[derive(Debug, PartialEq)]
	pub const MaxCollatorsPerDelegator: u32 = 1;
	/// Minimum stake required to be reserved to be a collator is 10_000
	pub const MinCollatorStake: Balance = 10_000 * PID;
	/// Minimum stake required to be reserved to be a delegator is 1000
	pub const MinDelegatorStake: Balance = constants::staking::MIN_DELEGATOR_STAKE;
	/// Maximum number of collator candidates
	#[derive(Debug, PartialEq)]
	pub const MaxCollatorCandidates: u32 = constants::staking::MAX_CANDIDATES;
	/// Maximum number of concurrent requests to unlock unstaked balance
	pub const MaxUnstakeRequests: u32 = 10;
	/// The starting block number for the network rewards
	pub const NetworkRewardStart: BlockNumber = INITIAL_PERIOD_LENGTH;
	/// The rate in percent for the network rewards
	pub const NetworkRewardRate: Perquintill = constants::staking::NETWORK_REWARD_RATE;
}

impl Config for Runtime {
    type Event = Event;
    type Currency = Balances;
    type CurrencyBalance = Balance;
    type MinBlocksPerRound = MinBlocksPerRound;
    type DefaultBlocksPerRound = DefaultBlocksPerRound;
    type StakeDuration = StakeDuration;
    type ExitQueueDelay = ExitQueueDelay;
    type MinCollators = MinCollators;
    type MinRequiredCollators = MinRequiredCollators;
    type MaxDelegationsPerRound = MaxDelegationsPerRound;
    type MaxDelegatorsPerCollator = MaxDelegatorsPerCollator;
    type MaxCollatorsPerDelegator = MaxCollatorsPerDelegator;
    type MinCollatorStake = MinCollatorStake;
    type MinCollatorCandidateStake = MinCollatorStake;
    type MaxTopCandidates = MaxCollatorCandidates;
    type MinDelegation = MinDelegatorStake;
    type MinDelegatorStake = MinDelegatorStake;
    type MaxUnstakeRequests = MaxUnstakeRequests;
    type NetworkRewardRate = NetworkRewardRate;
    type NetworkRewardStart = NetworkRewardStart;
    type NetworkRewardBeneficiary = Treasury;
    type WeightInfo = weights::parachain_staking::WeightInfo<Runtime>;
}