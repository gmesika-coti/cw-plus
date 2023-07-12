use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub enum ExecuteMsg {
/// Change the admin
VerifyProof { presentation: String },
}

#[cw_serde]
pub struct InstantiateMsg {

}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {

}
