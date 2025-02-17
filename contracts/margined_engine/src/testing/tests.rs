use crate::contract::{execute, instantiate, query};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{from_binary, Addr, Uint128};
use margined_common::asset::AssetInfo;
use margined_perp::margined_engine::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg};

const TOKEN: &str = "token";
const OWNER: &str = "owner";
const INSURANCE_FUND: &str = "insurance_fund";
const FEE_POOL: &str = "fee_pool";

#[test]
fn test_instantiation() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {
        decimals: 9u8,
        insurance_fund: INSURANCE_FUND.to_string(),
        fee_pool: FEE_POOL.to_string(),
        eligible_collateral: TOKEN.to_string(),
        initial_margin_ratio: Uint128::from(50_000_000u128), // 0.05
        maintenance_margin_ratio: Uint128::from(50_000_000u128), // 0.05
        liquidation_fee: Uint128::from(100u128),
    };
    let info = mock_info(OWNER, &[]);
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
    let config: ConfigResponse = from_binary(&res).unwrap();
    let info = mock_info(OWNER, &[]);
    assert_eq!(
        config,
        ConfigResponse {
            owner: info.sender,
            insurance_fund: Addr::unchecked(INSURANCE_FUND.to_string()),
            fee_pool: Addr::unchecked(FEE_POOL.to_string()),
            eligible_collateral: AssetInfo::Token {
                contract_addr: Addr::unchecked(TOKEN.to_string()),
            },
            decimals: Uint128::from(10u128.pow(9u32)),
            initial_margin_ratio: Uint128::from(50_000_000u128),
            maintenance_margin_ratio: Uint128::from(50_000_000u128),
            partial_liquidation_margin_ratio: Uint128::zero(),
            liquidation_fee: Uint128::from(100u128),
        }
    );

    // fails if decimals are 0
    let msg = InstantiateMsg {
        decimals: 0u8,
        insurance_fund: INSURANCE_FUND.to_string(),
        fee_pool: FEE_POOL.to_string(),
        eligible_collateral: TOKEN.to_string(),
        initial_margin_ratio: Uint128::from(50_000_000u128), // 0.05
        maintenance_margin_ratio: Uint128::from(50_000_000u128), // 0.05
        liquidation_fee: Uint128::from(100u128),
    };
    let info = mock_info(OWNER, &[]);
    let result = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap_err();
    assert_eq!(
        result.to_string(),
        "Generic error: Decimal places cannot be zero".to_string()
    );
}

#[test]
fn test_update_config() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {
        decimals: 9u8,
        insurance_fund: INSURANCE_FUND.to_string(),
        fee_pool: FEE_POOL.to_string(),
        eligible_collateral: TOKEN.to_string(),
        initial_margin_ratio: Uint128::from(50_000_000u128), // 0.05
        maintenance_margin_ratio: Uint128::from(50_000_000u128), // 0.05
        liquidation_fee: Uint128::from(100u128),
    };
    let info = mock_info(OWNER, &[]);
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // Update the config
    let msg = ExecuteMsg::UpdateConfig {
        owner: Some("addr0001".to_string()),
        insurance_fund: None,
        fee_pool: None,
        eligible_collateral: None,
        decimals: None,
        initial_margin_ratio: None,
        maintenance_margin_ratio: None,
        partial_liquidation_margin_ratio: None,
        liquidation_fee: None,
    };

    let info = mock_info(OWNER, &[]);
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
    let config: ConfigResponse = from_binary(&res).unwrap();
    assert_eq!(
        config,
        ConfigResponse {
            owner: Addr::unchecked("addr0001".to_string()),
            insurance_fund: Addr::unchecked(INSURANCE_FUND.to_string()),
            fee_pool: Addr::unchecked(FEE_POOL.to_string()),
            eligible_collateral: AssetInfo::Token {
                contract_addr: Addr::unchecked(TOKEN.to_string()),
            },
            decimals: Uint128::from(10u128.pow(9u32)),
            initial_margin_ratio: Uint128::from(50_000_000u128),
            maintenance_margin_ratio: Uint128::from(50_000_000u128),
            partial_liquidation_margin_ratio: Uint128::zero(),
            liquidation_fee: Uint128::from(100u128),
        }
    );

    // Update should fail
    let msg = ExecuteMsg::UpdateConfig {
        owner: Some(OWNER.to_string()),
        insurance_fund: None,
        fee_pool: None,
        eligible_collateral: None,
        decimals: None,
        initial_margin_ratio: None,
        maintenance_margin_ratio: None,
        partial_liquidation_margin_ratio: None,
        liquidation_fee: None,
    };

    let info = mock_info(OWNER, &[]);
    let result = execute(deps.as_mut(), mock_env(), info, msg);
    assert!(result.is_err());

    // Update should fail
    let msg = ExecuteMsg::UpdateConfig {
        owner: None,
        insurance_fund: None,
        fee_pool: None,
        eligible_collateral: None,
        decimals: None,
        initial_margin_ratio: Some(Uint128::MAX),
        maintenance_margin_ratio: None,
        partial_liquidation_margin_ratio: None,
        liquidation_fee: None,
    };

    let info = mock_info(OWNER, &[]);
    let result = execute(deps.as_mut(), mock_env(), info, msg);
    assert!(result.is_err());
}
