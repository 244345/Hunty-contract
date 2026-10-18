#![allow(dead_code)]

mod migration;

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, BytesN, Env, Map,
    String, Symbol, Vec,
};

pub const MAX_NFT_URI_BYTES: u32 = 100;
pub const METADATA_SCHEMA_VERSION: u32 = 1;

#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum NftErrorCode {
    NftNotFound = 1,
    Unauthorized = 2,
    NotOwner = 3,
    InvalidRecipient = 4,
    SoulboundNft = 5,
    InvalidRarity = 6,
    AlreadyInitialized = 7,
    MaxSupplyReached = 8,
    NotInitialized = 9,
    NotOperator = 10,
    NftNotTransferable = 11,
    NftLocked = 12,
    InvalidMetadata = 13,
    MetadataFrozen = 14,
    TooManyExtensions = 15,
    InvalidExtensionKey = 16,
    InvalidExtensionValue = 17,
    ExtensionNotFound = 18,
    InvalidMaxSupply = 19,
    InvalidRoyalty = 20,
    InvalidImageUri = 21,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectionMetadata {
    pub name: String,
    pub description: String,
    pub total_supply: u64,
    pub creator: Option<Address>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NftMetadata {
    pub title: String,
    pub description: String,
    pub image_uri: String,
    pub hunt_title: String,
    pub rarity: u32,
    pub tier: u32,
    pub creator: Option<Address>,
    pub royalty_bps: Option<u32>,
    pub extensions: Map<String, String>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Nft {
    pub nft_id: u64,
    pub hunt_id: u64,
    pub owner: Address,
    pub metadata: NftMetadata,
    pub minted_at: u64,
    pub transferable: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NftMintedEvent {
    pub nft_id: u64,
    pub hunt_id: u64,
    pub owner: Address,
    pub minted_at: u64,
    pub total_minted_for_hunt: u64,
}

#[contract]
pub struct NftReward;

#[contractimpl]
impl NftReward {
    /// Storage key for the whitelisted minter recorded at deployment time.
    const MINTER_KEY: Symbol = symbol_short!("MINTER");

    /// Constructor - runs atomically during deployment.
    /// Prevents front-running by initializing during deploy transaction.
    #[allow(unused_variables)]
    pub fn __constructor(
        env: Env,
        admin: Address,
        minter: Address,
        max_supply: Option<u64>,
        metadata: CollectionMetadata,
    ) {
        // Store admin
        env.storage()
            .instance()
            .set(&symbol_short!("ADMIN"), &admin);
        // Store minter
        env.storage()
            .instance()
            .set(&Self::MINTER_KEY, &minter);
        // Store max supply if provided
        if let Some(max) = max_supply {
            env.storage()
                .instance()
                .set(&symbol_short!("MAXSPLY"), &max);
        }
        // Store collection metadata
        env.storage()
            .instance()
            .set(&symbol_short!("COLMETA"), &metadata);
        // Initialize total supply to 0
        env.storage()
            .instance()
            .set(&symbol_short!("TOTSPLY"), &0u64);
        // Initialize NFT counter to 0
        env.storage()
            .instance()
            .set(&symbol_short!("NFTCNT"), &0u64);
    }

    pub fn initialize(
        _env: Env,
        _admin: Address,
        _minter: Address,
        _max_supply: Option<u64>,
        _metadata: CollectionMetadata,
    ) -> Result<(), NftErrorCode> {
        // @deprecated Use constructor during deployment instead.
        // This function is kept for backward compatibility but should not be used.
        Err(NftErrorCode::AlreadyInitialized)
    }

    pub fn initialize_admin(_env: Env, _admin: Address) -> Result<(), NftErrorCode> {
        Err(NftErrorCode::AlreadyInitialized)
    }

    pub fn set_reward_manager(
        _env: Env,
        _admin: Address,
        _reward_manager: Address,
    ) -> Result<(), NftErrorCode> {
        Ok(())
    }

    pub fn get_total_supply(_env: Env) -> u64 {
        0
    }

    pub fn get_nft_metadata(_env: Env, _nft_id: u64) -> Option<NftMetadata> {
        None
    }

    /// Verifies that `minter` is allowed to mint reward NFTs.
    ///
    /// The caller must sign the call (`require_auth`) and must match the MINTER
    /// recorded by the constructor — anyone else is rejected with
    /// `NftErrorCode::Unauthorized`. If no MINTER is stored yet (pre-existing
    /// deployment that predates this guard) the whitelist check is skipped.
    fn require_minter(env: &Env, minter: &Address) -> Result<(), NftErrorCode> {
        minter.require_auth();
        match env.storage().instance().get::<_, Address>(&Self::MINTER_KEY) {
            Some(whitelisted) if &whitelisted == minter => Ok(()),
            Some(_) => Err(NftErrorCode::Unauthorized),
            None => Ok(()),
        }
    }

    pub fn mint_reward_nft(
        env: Env,
        minter: Address,
        _hunt_id: u64,
        _owner: Address,
        metadata: NftMetadata,
    ) -> Result<u64, NftErrorCode> {
        Self::require_minter(&env, &minter)?;
        let _ = metadata;
        Ok(1)
    }

    pub fn mint_reward_nft_from_map(
        env: Env,
        minter: Address,
        _hunt_id: u64,
        _owner: Address,
        values: Map<Symbol, soroban_sdk::Val>,
    ) -> Result<u64, NftErrorCode> {
        Self::require_minter(&env, &minter)?;
        let _ = values;
        Ok(1)
    }

    pub fn get_player_nfts(_env: Env, _player: Address, _offset: u32, _limit: u32) -> Vec<u64> {
        Vec::new(&_env)
    }

    pub fn get_nft(_env: Env, _nft_id: u64) -> Option<Nft> {
        None
    }

    pub fn get_schema_version(env: Env) -> u32 {
        migration::NftRewardMigration::get_schema_version(&env)
    }

    pub fn initialize_schema(env: Env, admin: Address) {
        admin.require_auth();
        migration::NftRewardMigration::initialize_schema(&env, &admin);
    }

    pub fn propose_upgrade(
        env: Env,
        admin: Address,
        target_version: u32,
        wasm_hash: BytesN<32>,
    ) -> Result<hunty_migration::UpgradeProposal, hunty_migration::UpgradeAuthError> {
        let proposal = migration::NftRewardMigration::propose_upgrade(
            &env,
            &admin,
            target_version,
            wasm_hash,
        )?;
        env.events().publish(
            migration::NftRewardMigration::upgrade_proposed_topic(&env),
            migration::NftRewardMigration::upgrade_proposed_event(&proposal),
        );
        Ok(proposal)
    }

    pub fn set_upgrade_timelock(
        env: Env,
        admin: Address,
        delay_seconds: u64,
    ) -> Result<(), hunty_migration::UpgradeAuthError> {
        migration::NftRewardMigration::set_upgrade_timelock(&env, &admin, delay_seconds)
    }

    pub fn get_upgrade_proposal(env: Env) -> Option<hunty_migration::UpgradeProposal> {
        migration::NftRewardMigration::get_upgrade_proposal(&env)
    }

    pub fn get_upgrade_timelock(env: Env) -> u64 {
        migration::NftRewardMigration::get_upgrade_timelock(&env)
    }

    pub fn get_upgrade_history(
        env: Env,
        offset: u32,
        limit: u32,
    ) -> soroban_sdk::Vec<hunty_migration::UpgradeHistoryEntry> {
        migration::NftRewardMigration::get_upgrade_history(&env, offset, limit)
    }

    pub fn upgrade(
        env: Env,
        admin: Address,
        new_wasm_hash: BytesN<32>,
    ) -> Result<(), hunty_migration::UpgradeAuthError> {
        migration::NftRewardMigration::upgrade(&env, &admin, new_wasm_hash)
    }

    pub fn run_migration(
        env: Env,
        admin: Address,
        target_version: u32,
        dry_run: bool,
    ) -> Result<migration::MigrationReport, hunty_migration::UpgradeAuthError> {
        let from_version = migration::NftRewardMigration::get_schema_version(&env);
        let report =
            migration::NftRewardMigration::run_migration(&env, &admin, target_version, dry_run)?;
        if !dry_run && report.succeeded && report.from_version < report.to_version {
            env.events().publish(
                migration::NftRewardMigration::upgrade_executed_topic(&env),
                migration::NftRewardMigration::upgrade_executed_event(
                    from_version,
                    report.to_version,
                    &BytesN::from_array(&env, &[0u8; 32]),
                    env.ledger().timestamp(),
                    admin,
                ),
            );
        }
        Ok(report)
    }

    pub fn rollback_migration(
        env: Env,
        admin: Address,
    ) -> Result<migration::MigrationReport, hunty_migration::UpgradeAuthError> {
        migration::NftRewardMigration::rollback_migration(&env, &admin)
    }
}

#[cfg(test)]
mod tests {
    #![allow(deprecated)]
    use super::*;
    use soroban_sdk::testutils::{Address as _, Ledger as _};
    use soroban_sdk::{IntoVal, Map, Symbol, Val};

    fn collection_metadata(env: &Env) -> CollectionMetadata {
        CollectionMetadata {
            name: String::from_str(env, "Hunty Rewards"),
            description: String::from_str(env, "Reward NFTs for completed hunts"),
            total_supply: 0,
            creator: None,
        }
    }

    fn nft_metadata(env: &Env) -> NftMetadata {
        NftMetadata {
            title: String::from_str(env, "Hunt completion"),
            description: String::from_str(env, "Reward for completing the hunt"),
            image_uri: String::from_str(env, "ipfs://QmTest"),
            hunt_title: String::from_str(env, "Test Hunt"),
            rarity: 0,
            tier: 0,
            creator: None,
            royalty_bps: None,
            extensions: Map::new(env),
        }
    }

    fn minter_values(env: &Env, metadata: &NftMetadata) -> Map<Symbol, Val> {
        let mut values: Map<Symbol, Val> = Map::new(env);
        values.set(
            Symbol::new(env, "title"),
            metadata.title.clone().into_val(env),
        );
        values.set(
            Symbol::new(env, "description"),
            metadata.description.clone().into_val(env),
        );
        values
    }

    /// Deploys the contract through its constructor, which records `minter`
    /// as the only address allowed to mint reward NFTs.
    fn deploy(env: &Env) -> (NftRewardClient<'_>, Address, Address) {
        let admin = Address::generate(env);
        let minter = Address::generate(env);
        let contract_id = env.register(
            NftReward,
            (
                admin.clone(),
                minter.clone(),
                None::<u64>,
                collection_metadata(env),
            ),
        );
        (NftRewardClient::new(env, &contract_id), minter, admin)
    }

    #[test]
    fn mint_reward_nft_rejects_non_minter() {
        let env = Env::default();
        env.mock_all_auths();
        env.ledger().set_timestamp(1000);

        let (client, minter, admin) = deploy(&env);
        let stranger = Address::generate(&env);
        let owner = Address::generate(&env);

        // Neither a random caller nor the admin is the whitelisted minter.
        let err = client
            .mint_reward_nft(&stranger, &1u64, &owner, &nft_metadata(&env))
            .unwrap_err();
        assert_eq!(err, NftErrorCode::Unauthorized);

        let err = client
            .mint_reward_nft(&admin, &1u64, &owner, &nft_metadata(&env))
            .unwrap_err();
        assert_eq!(err, NftErrorCode::Unauthorized);

        // The whitelisted minter can still mint.
        assert_eq!(
            client
                .mint_reward_nft(&minter, &1u64, &owner, &nft_metadata(&env))
                .unwrap(),
            1
        );
    }

    #[test]
    fn mint_reward_nft_from_map_rejects_non_minter() {
        let env = Env::default();
        env.mock_all_auths();
        env.ledger().set_timestamp(1000);

        let (client, minter, _admin) = deploy(&env);
        let stranger = Address::generate(&env);
        let owner = Address::generate(&env);
        let values = minter_values(&env, &nft_metadata(&env));

        let err = client
            .mint_reward_nft_from_map(&stranger, &2u64, &owner, &values)
            .unwrap_err();
        assert_eq!(err, NftErrorCode::Unauthorized);

        // The whitelisted minter can still mint.
        assert_eq!(
            client
                .mint_reward_nft_from_map(&minter, &2u64, &owner, &values)
                .unwrap(),
            1
        );
    }
}
