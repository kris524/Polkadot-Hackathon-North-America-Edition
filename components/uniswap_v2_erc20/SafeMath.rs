

pub enum MathErrors {
    MathAddOverflow,
    MathSubUnderflow,
    MathMulOverflow,
}

pub type MathResult<T> = core::result::Result<T, MathErrors>;


use ethereum_types::U256;
pub mod SafeMath {
    fn add(x: U256, y: U256) -> MathResult<()> {
        let z = x+y;

        if z >= x{
            return Ok(z);
        }
        else{
            return Err(MathErrors::MathAddOverflow);
        }

    }
    fn sub(x: U256, y: U256) -> MathResult<()>{
        let z = x-y;

        if z <= x{
            return Ok(z);
        }
        else{
            return Err(MathErrors::MathSubUnderflow);
        }
    }

    fn mul(x: U256, y: U256) -> MathResult<()>{
        let z = x*y;

        if y ==0 || (z/y) == x {
            return Ok(z);
        }
        else{
            return Err(MathErrors::MathMulOverflow);
        }
    }
}