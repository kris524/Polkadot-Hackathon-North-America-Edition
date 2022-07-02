

use ethereum_types;
use uint::construct_uint;

construct_uint! {
    pub struct U112(16);
    pub struct U1224(16);
}

pub mod UQ112x112 {
    const Q112: U224 = 2**112;

    fn encode(y: U112) -> U112 {


        let z: U224 = (y as U224) * Q112;
        return z;
    }
    fn uqdiv(x: U224, y: U112) -> U224 {
        let z = x / (y as U224);
        return z;
    }
}