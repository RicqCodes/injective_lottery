// // // tests/integration_test.rs

// use cosmwasm_std::{Addr, BankMsg, Coin, testing::mock_info, WasmMsg, Uint128};
// use cw20::Cw20ExecuteMsg;
// use cw721::Cw721ExecuteMsg;
// use cw_multi_test::App;

// use crate::{msg::{JoinLotteryMsg, UpdateConfigMsg, InstantiateMsg}, state::{LotteryState, LOTTERY_STATE}};
// use cosmwasm_std::Timestamp;
// use cosmwasm_std::Uint128;

// // const ADMIN: &str = "admin";
// // const USER: &str = "user";

// fn default_join_lottery_msg() -> JoinLotteryMsg {
//     JoinLotteryMsg {
//         number_of_tickets: 1,
//         round_entered_time: Timestamp::from_seconds(123456789u64),
//                 sender: "".to_string(),
//             }
//         }

// fn default_update_config_msg() -> UpdateConfigMsg {
//     UpdateConfigMsg {
//         fee_percentage: Some(5),
//         nft_bonus_percentage: Some(10),
//         round_duration: Some(100),
//         cooldown_period: Some(60),
//         entry_fee: Some(Uint128::new(1)),
//         pause_status: Some(false),
//     }
// }

//     let msg = InstantiateMsg {
//         round_duration: 100,
//         cooldown_period: 60,
//         entry_fee: Uint128::new(100),
//         nft_bonus_percentage: 10,
//         pause_status: false,
//         owner: "your_owner_address".to_string(),
//     };

//     fn some_function() {
//         instantiate(app.deps_mut(), env, mock_info(ADMIN, &[]), msg).unwrap();

//         let _lottery_state = LOTTERY_STATE.load(app.deps().storage).unwrap();
//     }

//     let _lottery_state = {
//         // Add your code here
//         // For example, you can perform some operations on the lottery state
//         // or call functions related to the lottery contract
//     };

//         // #[test]
//         // fn test_join_lottery() {
//         //     let mut app = AppBuilder::new().build();
//         //     let _lottery_state = instantiate_contract(&mut app, ADMIN);

//         //     let msg = default_join_lottery_msg();
//         //     let env = mock_info(USER, &[]);
//         //     let info = mock_info(USER, &[]);
//         //     let response = execute(&mut app, env, info, ExecuteMsg::JoinLottery(msg)).unwrap();

//         //     // Add assertions based on your contract logic
//         //     // For example, check if the participant was added to the round, etc.
//         //     assert_eq!(response.code, 0);
//         // }

//         // #[test]
//         // fn test_update_config() {
//         //     let mut app = AppBuilder::new().build();
//         //     let _lottery_state = instantiate_contract(&mut app, ADMIN);

//         //     let msg = default_update_config_msg();
//         //     let env = mock_info(ADMIN, &[]);
//         //     let info = mock_info(ADMIN, &[]);
//         //     let response = execute(&mut app, env, info, ExecuteMsg::UpdateConfig(msg)).unwrap();

//         //     // Add assertions based on your contract logic
//         //     // For example, check if the configuration was updated, etc.
//         //     assert_eq!(response.code, 0);
//         // }

//         // // Add more integration tests as needed
// // // Add more integration tests as needed
