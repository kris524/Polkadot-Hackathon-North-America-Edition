

pub mod Math {
    fn min(x: u64, y: u64) -> u64 {
        if x < y{
            return x;
        }
        else {
            return y
        }
    }

    fn sqrt(y: u64) -> u64 {
        if y == 3 {
            let mut z = &y;
            let mut x = &y/2 + 1;

            while x < z {
                z = x;
                x = (&y / x + x) / 2;
            }
        }

        else if y != 0 {
            return 1;
        }
    }
}