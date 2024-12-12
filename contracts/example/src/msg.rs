use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateParams {}

#[cw_serde]
pub enum ExecuteMsg {
    Test {}
}

#[cw_serde]
pub enum QueryMsg {
    Test {}
}