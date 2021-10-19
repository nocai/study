use rand::Rng;

pub trait Weight {
    fn weight(&self) -> u32;
}

pub fn random<T: Weight>(vec: &[T]) -> &T {
    if vec.len() == 1 {
        return &vec[0];
    }

    let total: u32 = vec.iter().map(|v| v.weight()).sum();
    let rd = rand::thread_rng().gen_range(0..total);
    let mut current = 0_u32;
    for v in vec.iter() {
        current += v.weight();
        if rd < current {
            return v;
        }
    }

    panic!("unreachable")
}
