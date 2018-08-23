pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.len() == 0 {
        return Vec::new();
    }

    let mut row_max: Vec<u64> = Vec::new();
    row_max.resize(input.len(), 0);
    let mut col_min: Vec<u64> = Vec::new();
    col_min.resize(input[0].len(), std::u64::MAX);

    let v: Vec<(usize, usize, u64)> = input.iter()
        .enumerate()
        .flat_map(|(i, r)| {
            r.iter()
                .enumerate()
                .map(move |(j, val)| (i, j, *val))
        }).collect();

    v.iter()
        .for_each(|(i, j, val)| {
            if row_max[*i] < *val {
                row_max[*i] = *val;
            }
            if col_min[*j] > *val {
                col_min[*j] = *val;
            }
        });

    v.iter()
        .filter(|(i, j, val)| *val == row_max[*i] && *val == col_min[*j])
        .map(|(i, j, _)| (*i, *j))
        .collect()
}
