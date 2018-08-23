struct Primes {
    primes: Vec<u32>
}

impl Primes {
    fn new() -> Primes {
        Primes {
            primes: [3, 5].to_vec()
        }
    }

    fn next(&mut self) -> u32 {
        // construct a segmented seive of eratasthones
        // starting after the last known prime
        let mut last: u32 = (*self.primes.last().unwrap()).clone() + 1;
        let unfound = true;

        while unfound {
            // use the next 64 numbers for the segment
            let end = last + 64;
            let mut search = [true; 64];

            for p in &self.primes {
                let mut n = ((last / p) * p) as i32 - last as i32;
                if n < 0 { n += *p as i32; }

                while n < 64 {
                    search[n as usize] = false;
                    n += *p as i32;
                }
            }

            for (i, &n) in search.iter().enumerate() {
                if n {
                    let q = i as u32 + last;
                    self.primes.push(q);
                    return q;
                }
            }

            last = end;
        }

        // have to add this so that rustc will have a return value
        // for all possible code paths
        0
    }

    fn last(&self) -> Option<u32> {
        Some(self.primes.last().unwrap().clone())
    }
}

pub fn nth(n: u32) -> Option<u32> {
    if n == 0 {
        None
    } else if n == 1 {
        Some(2)
    } else if n == 2 {
        Some(3)
    } else {
        let mut p = Primes::new();
        for _ in 2..n {
            p.next();
        }

        p.last()
    }
}
