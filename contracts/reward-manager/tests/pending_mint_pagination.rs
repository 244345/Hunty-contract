use reward_manager::storage::Storage;
use reward_manager::{PendingNftMint, RewardManager};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Env, String};

#[test]
fn oversized_limit_saturates_and_is_capped() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let xlm_token = Address::generate(&env);
    let hunty_core = Address::generate(&env);
    let contract_id = env.register(RewardManager, (admin, xlm_token, hunty_core));

    env.as_contract(&contract_id, || {
        for hunt_id in 0..52 {
            let player = Address::generate(&env);
            let pending = PendingNftMint {
                hunt_id,
                player: player.clone(),
                nft_contract: Address::generate(&env),
                nft_title: String::from_str(&env, "Reward"),
                nft_description: String::from_str(&env, "Description"),
                nft_image_uri: String::from_str(&env, "https://example.com/reward.png"),
                nft_hunt_title: String::from_str(&env, "Hunt"),
                nft_rarity: 0,
                nft_tier: 0,
                completion_rank: 0,
            };
            Storage::set_pending_nft_mint(&env, hunt_id, &player, &pending);
        }

        let page = Storage::list_pending_nft_mints(&env, 1, u32::MAX);
        assert_eq!(page.len(), Storage::MAX_PENDING_NFT_PAGE_SIZE);
    });
}