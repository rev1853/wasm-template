use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, coin, Decimal, Uint128};
use cw20::{Cw20Coin, Denom};

use crate::modules::decimal::DecimalExt;

#[cw_serde]
pub struct Asset {
    pub denom: Denom,
    pub amount: Uint128
}

impl Asset {
    pub fn is_cw20(&self) -> bool {
        return match self.denom {
            Denom::Native(_) => false,
            Denom::Cw20(_) => true
        }
    }

    pub fn cw20_address(&self) -> Option<String> {
        return match self.denom.clone() {
            Denom::Native(_) => None,
            Denom::Cw20(address) => Some(address.to_string())
        }
    }

    pub fn coin(&self) -> Option<Coin> {
        return match self.denom.clone() {
            Denom::Native(denom) => Some(coin(self.amount.into(), denom)),
            Denom::Cw20(_) => None,
        }
    }

    pub fn spendable(self, is_classic: bool) -> Self {
        return if is_classic && !self.is_cw20() {
            self.clone().multiply_decimal(&Decimal::from_str("0.995").unwrap())
        } else {
            self
        }
    }
}

impl From<Coin> for Asset {
    fn from(value: Coin) -> Self {
        return Asset {
            denom: Denom::Native(value.denom),
            amount: value.amount
        }
    }
}

impl From<Cw20Coin> for Asset {
    fn from(value: Cw20Coin) -> Self {
        return Asset {
            denom: Denom::Cw20(Addr::unchecked(value.address)),
            amount: value.amount
        }
    }
}

pub trait AssetOperation {
    fn divide_decimal(self, decimal: &Decimal) -> Asset;
    fn multiply_decimal(self, decimal: &Decimal) -> Asset;
    fn plus(self, other: &Asset) -> Asset;
    fn minus(self, other: &Asset) -> Asset;
}

impl AssetOperation for Asset {
    fn divide_decimal(mut self, decimal: &Decimal) -> Self {
        self.amount = Decimal::from_num(self.amount).div(decimal).to_uint_floor();
        self
    }

    fn multiply_decimal(mut self, decimal: &Decimal) -> Self {
        self.amount = Decimal::from_num(self.amount).mul(decimal).to_uint_floor();
        self
    }

    fn plus(mut self, other: &Asset) -> Self {
        if other.denom.eq(&other.denom) {
            self.amount = self.amount.add(other.amount);
        }
        self
    }

    fn minus(mut self, other: &Asset) -> Self {
        if other.denom.eq(&other.denom) {
            self.amount = self.amount.sub(other.amount);
        }
        self
    }
}

#[cw_serde]
pub struct Assets {
   pub assets: Vec<Asset>
}

impl From<Vec<Coin>> for Assets {
    fn from(value: Vec<Coin>) -> Self {
        return Assets {
            assets: value.iter().map(|el| Asset::from(el.clone())).collect()
        }
    }
}