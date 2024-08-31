use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use crate::error::ContractResult;

pub trait ExecuteHandler<T, S> {
    fn handle(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        state: S,
        msg: T
    ) -> ContractResult<Response>;
}
