use hunty_core::{BatchClueInput, HuntyCore, HuntyCoreClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{vec, Address, Env, String};

#[test]
fn test_difficulty_rating_updates() {
    let env = Env::default();
    env.mock_all_auths();

    let creator = Address::generate(&env);
    let core_id = env.register(HuntyCore, ());
    let client = HuntyCoreClient::new(&env, &core_id);

    let hunt_id = client.create_hunt(
        &creator,
        &String::from_str(&env, "Difficulty Test Hunt"),
        &String::from_str(&env, "Testing difficulty rating updates"),
        &None,
        &None,
        &0,
        &None,
        &None,
    );

    let hunt = client.get_hunt(&hunt_id);
    assert_eq!(hunt.difficulty_rating, 0);

    // Add first clue with difficulty 2
    client.add_clue(
        &hunt_id,
        &String::from_str(&env, "Q1"),
        &String::from_str(&env, "A1"),
        &10,
        &true,
        &Some(2),
        &None,
    );

    let hunt = client.get_hunt(&hunt_id);
    assert_eq!(hunt.difficulty_rating, 2);

    // Add second clue with difficulty 4
    client.add_clue(
        &hunt_id,
        &String::from_str(&env, "Q2"),
        &String::from_str(&env, "A2"),
        &10,
        &true,
        &Some(4),
        &None,
    );

    let hunt = client.get_hunt(&hunt_id);
    assert_eq!(hunt.difficulty_rating, 3); // (2 + 4) / 2 = 3

    // Add multiple clues via add_clues
    let clues = vec![
        &env,
        BatchClueInput {
            question: String::from_str(&env, "Q3"),
            answer: String::from_str(&env, "A3"),
            points: 10,
            is_required: true,
            difficulty: 5,
        },
        BatchClueInput {
            question: String::from_str(&env, "Q4"),
            answer: String::from_str(&env, "A4"),
            points: 10,
            is_required: true,
            difficulty: 1,
        },
    ];

    client.add_clues(&hunt_id, &clues);

    let hunt = client.get_hunt(&hunt_id);
    // Average of 2, 4, 5, 1 is 12 / 4 = 3
    assert_eq!(hunt.difficulty_rating, 3);
}
