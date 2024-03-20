pub struct PascalsTriangle(u32);

// rewriter this with Cell, inner state and iterator. 
impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let n = self.0 as usize;
        let mut res = Vec::with_capacity(n);
        // this iterator is my attempt to make function below more consice.
        (0..n).iter().for_each(|_|{
            let last 
        }).collect()
        if n > 0 {
            for row in 0..n {
            // first two rows are constant
            if row == 0 {res.push(vec![1u32])} else if row == 1 {
                res.push(vec![1u32, 1u32])}
            // all other will have to be calculated
            else {
                let mut middle =  (row + 1) / 2;
                if row % 2 == 0 { middle += 1;};
                
                let mut half:Vec<u32> = Vec::with_capacity(middle);
                let last = res.last().unwrap(); // last always not None.
                for idx in 0 .. middle {
                    // first is always 1
                    if idx == 0 { 
                        half.push(1);
                        continue;
                    }
                    let first = last.get(idx - 1).unwrap_or(&0u32) ;
                    let second = last.get(idx).unwrap_or(&0u32);
                    half.push(first + second);
                    }
                // must copy because later need to extend anyway
                let mut cloned = half.clone();
                if row % 2 == 0{
                    let _x = cloned.pop();
                }
                cloned.reverse();
                half.extend(cloned.iter());
                res.push(half);

                }
            }
        }
        res
    }
}
