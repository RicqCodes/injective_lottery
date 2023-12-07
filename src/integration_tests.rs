// // tests/integration_test.rs

// use cosmwasm_std::{Addr, BankMsg, Coin, testing::mock_info, WasmMsg, Uint128};
// use cw20::Cw20ExecuteMsg;
// use cw20_base::msg::InstantiateMsg as Cw20InstantiateMsg;
// use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

// use lottery::{execute, instantiate, query, state::LotteryState, ExecuteMsg, InstantiateMsg, JoinLotteryMsg, UpdateConfigMsg};

// const ADMIN: &str = "admin";
// const USER: &str = "user";

// fn default_join_lottery_msg() -> JoinLotteryMsg {
//     JoinLotteryMsg {
//         number_of_tickets: 1,
//         round_entered_time: 123456789, // Replace with actual time
//     }
// }

// fn default_update_config_msg() -> UpdateConfigMsg {
//     UpdateConfigMsg {
//         fee_percentage: Some(5),
//         nft_bonus_percentage: Some(10),
//         round_duration: Some(100),
//         cooldown_period: Some(60),
//         entry_fee: Uint128(1),
//         pause_status: Some(false),
//     }
// }

// fn instantiate_contract(app: &mut App, owner: &str) -> LotteryState {
//     let msg = InstantiateMsg {
//         round_duration: 100,
//         cooldown_period: 60,
//         fee_percentage: 5,
//         entry_fee: 100,
//         nft_bonus_percentage: 10,
//         pause_status: false,
//     };
//     let env = mock_info(owner, &[]);
//     instantiate(app.deps_mut(), env, mock_info(ADMIN, &[]), msg).unwrap();

//     LOTTERY_STATE.load(app.deps().storage).unwrap()
// }

// #[test]
// fn test_join_lottery() {
//     let mut app = AppBuilder::new().build();
//     let _lottery_state = instantiate_contract(&mut app, ADMIN);

//     let msg = default_join_lottery_msg();
//     let env = mock_info(USER, &[]);
//     let info = mock_info(USER, &[]);
//     let response = execute(&mut app, env, info, ExecuteMsg::JoinLottery(msg)).unwrap();

//     // Add assertions based on your contract logic
//     // For example, check if the participant was added to the round, etc.
//     assert_eq!(response.code, 0);
// }

// #[test]
// fn test_update_config() {
//     let mut app = AppBuilder::new().build();
//     let _lottery_state = instantiate_contract(&mut app, ADMIN);

//     let msg = default_update_config_msg();
//     let env = mock_info(ADMIN, &[]);
//     let info = mock_info(ADMIN, &[]);
//     let response = execute(&mut app, env, info, ExecuteMsg::UpdateConfig(msg)).unwrap();

//     // Add assertions based on your contract logic
//     // For example, check if the configuration was updated, etc.
//     assert_eq!(response.code, 0);
// }

// // Add more integration tests as needed
