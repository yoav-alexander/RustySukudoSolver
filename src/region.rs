use cached::proc_macro::cached;

#[derive(Copy, Clone, Debug)]
pub enum RegionType {
    Row,
    Col,
    Box,
}

#[cached]
pub fn get_all_boxes(size: usize) -> Vec<Vec<(usize, usize)>> {
    let mut regions = Vec::new();
    let block_size = size.isqrt();

    let box_vector_map: Vec<(usize, usize)> = (0..block_size)
        .flat_map(|i| (0..block_size).map(|j| (i, j)).collect::<Vec<_>>())
        .collect();

    for bi in 0..block_size {
        for bj in 0..block_size {
            let box_region = box_vector_map
                .iter()
                .map(|&(i, j)| (bi * block_size + i, bj * block_size + j))
                .collect();
            regions.push(box_region);
        }
    }
    regions
}

#[cached]
pub fn get_all_regions(size: usize) -> Vec<(RegionType, Vec<(usize, usize)>)> {
    let mut regions = Vec::new();

    for i in 0..size {
        regions.push((RegionType::Row, (0..size).map(|j| (i, j)).collect()));
        regions.push((RegionType::Col, (0..size).map(|j| (j, i)).collect()));
    }

    regions.extend(
        get_all_boxes(size)
            .into_iter()
            .map(|v| (RegionType::Box, v)),
    );
    regions
}
