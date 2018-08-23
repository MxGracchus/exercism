pub fn find() -> Option<u32> {
    for i in 1..1000 {
        for j in (i + 1)..1000 {
            let (a, b, c) = match i * i + j * j {
                c if c % 2 == 0 => (
                        (j * j - i * i) / 2,
                        (2 * i * j) / 2,
                        c / 2
                    ),
                c => (
                        j * j - i * i,
                        2 * i * j,
                        c
                    ),

            };

            let s = a + b + c;
            if s == 1000 {
                return Some(a * b * c);
            } else if s > 1000 {
                break;
            } else if 1000 % s == 0 {
                let d = 1000 / s;
                return Some((d * d * d) * (a * b * c));
            }
        }
    }

    None
}
