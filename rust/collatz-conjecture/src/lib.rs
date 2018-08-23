pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        None
    } else {
        let mut c: u64 = n;
        let mut i: u64 = 0;

        while c != 1 {
            if c % 2 == 0 {
                c = c / 2;
            } else {
                c = 3 * c + 1;
            }

            i += 1;
        }

        Some(i)
    }
}
