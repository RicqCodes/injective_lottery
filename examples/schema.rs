use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use lottery::msg::{ExecuteMsg, InstantiateMsg, JoinLotteryMsg, QueryMsg, RoundInfoQuery};
use lottery::state::LotteryState;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(LotteryState), &out_dir);
    export_schema(&schema_for!(JoinLotteryMsg), &out_dir);
    export_schema(&schema_for!(RoundInfoQuery), &out_dir);
    
}
