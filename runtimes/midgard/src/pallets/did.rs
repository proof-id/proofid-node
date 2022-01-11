// --- paritytech ---
use pallet_did::Config;

// --- proofid-network ---
use crate::*;

parameter_types! {
	pub const MaxNewKeyAgreementKeys: u32 = MAX_KEY_AGREEMENT_KEYS;
	#[derive(Debug, Clone, PartialEq)]
	pub const MaxUrlLength: u32 = MAX_URL_LENGTH;
	pub const MaxPublicKeysPerDid: u32 = MAX_PUBLIC_KEYS_PER_DID;
	#[derive(Debug, Clone, PartialEq)]
	pub const MaxTotalKeyAgreementKeys: u32 = MAX_TOTAL_KEY_AGREEMENT_KEYS;
	#[derive(Debug, Clone, PartialEq)]
	pub const MaxEndpointUrlsCount: u32 = MAX_ENDPOINT_URLS_COUNT;
	// Standalone block time is half the duration of a parachain block.
	pub const MaxBlocksTxValidity: BlockNumber = MAX_BLOCKS_TX_VALIDITY;
	pub const DidDeposit: Balance = DID_DEPOSIT;
	pub const DidFee: Balance = DID_FEE;
	pub const MaxNumberOfServicesPerDid: u32 = MAX_NUMBER_OF_SERVICES_PER_DID;
	pub const MaxServiceIdLength: u32 = MAX_SERVICE_ID_LENGTH;
	pub const MaxServiceTypeLength: u32 = MAX_SERVICE_TYPE_LENGTH;
	pub const MaxServiceUrlLength: u32 = MAX_SERVICE_URL_LENGTH;
	pub const MaxNumberOfTypesPerService: u32 = MAX_NUMBER_OF_TYPES_PER_SERVICE;
	pub const MaxNumberOfUrlsPerService: u32 = MAX_NUMBER_OF_URLS_PER_SERVICE;
}

impl Config for Runtime {
    type DidIdentifier = DidIdentifier;
    type Event = Event;
    type Call = Call;
    type Origin = Origin;
    type Currency = Balances;
    type Deposit = DidDeposit;
    type Fee = DidFee;
    type FeeCollector = Treasury;

    #[cfg(not(feature = "runtime-benchmarks"))]
    type EnsureOrigin = pallet_did::EnsureDidOrigin<DidIdentifier, AccountId>;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type OriginSuccess = pallet_did::DidRawOrigin<AccountId, DidIdentifier>;

    #[cfg(feature = "runtime-benchmarks")]
    type EnsureOrigin = EnsureSigned<DidIdentifier>;
    #[cfg(feature = "runtime-benchmarks")]
    type OriginSuccess = DidIdentifier;

    type MaxNewKeyAgreementKeys = MaxNewKeyAgreementKeys;
    type MaxTotalKeyAgreementKeys = MaxTotalKeyAgreementKeys;
    type MaxPublicKeysPerDid = MaxPublicKeysPerDid;
    type MaxBlocksTxValidity = MaxBlocksTxValidity;
    type MaxNumberOfServicesPerDid = MaxNumberOfServicesPerDid;
    type MaxServiceIdLength = MaxServiceIdLength;
    type MaxServiceTypeLength = MaxServiceTypeLength;
    type MaxServiceUrlLength = MaxServiceUrlLength;
    type MaxNumberOfTypesPerService = MaxNumberOfTypesPerService;
    type MaxNumberOfUrlsPerService = MaxNumberOfUrlsPerService;
    type WeightInfo = weights::did::WeightInfo<Runtime>;
}