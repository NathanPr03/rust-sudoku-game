//This is slower but cba writing it by hand
pub fn sixteen_by_sixteen_cover_matrix() -> Vec<Vec<u8>> {
    let mut cover_matrix: Vec<Vec<u8>> = Vec::new();

    for _i in 0..4096 {
        let mut row: Vec<u8> = Vec::new();
        for _j in 0..1024 {
            row.push(0);
        }
        cover_matrix.push(row);
    }

    return cover_matrix;
}