use anchor_lang::{prelude::*, solana_program::log::sol_log_compute_units};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

use uint::construct_uint;
construct_uint! {
    pub struct U128(2);
}

//[...]
pub fn compare_compute_budget_consumption() {
    let x = 17234u64; //some arbitrary number
    let y = 3273u64; //some arbitrary number
    
    //u64
    let v1 = x*10u64.pow(9);
    let v2 = y;
    sol_log_compute_units();
    let _add = v1+v2;
    sol_log_compute_units();
    let _sub = v1-v2;
    sol_log_compute_units();
    let _mul = v1*v2;
    sol_log_compute_units();
    let _div = v1/v2;
    sol_log_compute_units();
    
    //u128
    let v1 = (x as u128)*10u128.pow(19);
    let v2 = y as u128;
    sol_log_compute_units();
    let _add = v1+v2;
    sol_log_compute_units();
    let _sub = v1-v2;
    sol_log_compute_units();
    let _mul = v1*v2;
    sol_log_compute_units();
    let _div = v1/v2;
    sol_log_compute_units();
    
    //U128
    let v1 = U128::from(x)*U128::from(10).pow(19.into());
    let v2 = U128::from(y);
    sol_log_compute_units();
    let _add = v1+v2;
    sol_log_compute_units();
    let _sub = v1-v2;
    sol_log_compute_units();
    let _mul = v1*v2;
    sol_log_compute_units();
    let _div = v1/v2;
    sol_log_compute_units();
}

#[program]
pub mod sol_u128 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        compare_compute_budget_consumption();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
