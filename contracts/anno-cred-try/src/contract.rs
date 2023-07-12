use std::fmt;
use anoncreds::data_types::presentation;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Storage};
use cw2::{CONTRACT, ContractVersion};
use anoncreds::verifier;


// use cw2::set_contract_version;

use crate::error::ContractError;
// use crate::annoMsg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::annoMsg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use log::{Record, Level, Metadata, LevelFilter, SetLoggerError};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:anno-cred-try";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw4-group";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    unimplemented!()
}

pub fn set_contract_version<T: Into<String>, U: Into<String>>(
    store: &mut dyn Storage,
    name: T,
    version: U,
) -> StdResult<()> {
    let val = ContractVersion {
        contract: name.into(),
        version: version.into(),
    };
    CONTRACT.save(store, &val)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let api = _deps.api;
    match _msg {
        ExecuteMsg::VerifyProof { presentation } => {
            // let presentation_string = String::from_utf8(presentation).unwrap();
            log::debug!("presentation: {}", &presentation);
            Ok(Response::new().add_attribute("f","x"))
        }
    }
    // let valid = verifier::verify_presentation( // this needed in the contract
    //                                            &presentation,
    //                                            &pres_request,
    //                                            &schemas,
    //                                            &cred_defs,
    //                                            None,
    //                                            None,
    //                                            None,
    // );
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
