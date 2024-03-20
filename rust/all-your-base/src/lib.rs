#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

fn decimal_to_digits_base(mut n: u32, base: u32) -> Result<Vec<u32>, Error> {
    let mut v: Vec<u32> = vec![];
    if n == 0 {
        return Ok(vec![0u32]);
    }

    if base.ge(&10) {
        let mut num = n;
        while num.ge(&base) {
            let pow: u32 = ((num as f32).log(base as f32)) as u32;
            let div = base.pow(pow);
            let n = num / div;
            v.push(n);
            let reducer = div * n;
            num -= reducer;
        }

        if num > 0 {
            v.push(num);
        }
    } else {
        let float_base = base as f32;
        let mut size: usize = 1;
        let float_n = n as f32;
        let first_take = float_n.log(float_base) as usize;
        if first_take > 0 {
            size = first_take + 1;
        }
        let mut vct: Vec<u32> = vec![0; size];
        if first_take > 0 {
            if first_take == 1 {
                let q = n / base;
                vct[0] = q;
                n -= q * base;
            } else {
                vct[0] = 1;
                n -= base.pow(first_take as u32);
            };
        }
        while n >= base {
            let take = (n as f32).log(float_base) as usize;
            if take > 0 {
                if take == 1 && base > 2 {
                    let idx = (size - 1) - take;
                    vct[idx] = n / base;
                    n = 0;
                } else {
                    let idx = (size - 1) - take;

                    n -= base.pow(take as u32);
                    vct[idx] = 1;
                }
            }
        }

        if n > 0 {
            vct[size - 1] = n;
        } else {
            vct[size - 1] = 0;
        }
        v = vct;
    }
    Ok(v)
}

fn from_base_to_decimal(nums: &[u32], from_base: u32) -> Result<u32, Error> {
    if nums.is_empty() {
        return Ok(0);
    }
    let mut sum = 0;
    for (idx, n) in nums.iter().enumerate() {
        if from_base.eq(&2) {
            if n.eq(&1) {
                let p = ((nums.len() - 1) - idx) as u32;
                sum += from_base.pow(p);
            } else if n.gt(&1) {
                return Err(Error::InvalidDigit(*n));
            }
        } else {
            let p = ((nums.len() - 1) - idx) as u32;
            sum += n * from_base.pow(p);
        }
    }
    Ok(sum)
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if to_base.eq(&0) || to_base.eq(&1) {
        return Err(Error::InvalidOutputBase);
    }

    if from_base.eq(&0) || from_base.eq(&1) {
        return Err(Error::InvalidInputBase);
    }
    let dcm = from_base_to_decimal(number, from_base);
    if let Ok(dcm) = dcm {
        decimal_to_digits_base(dcm, to_base)
    } else {
        Err(dcm.unwrap_err())
    }
}
