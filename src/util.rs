use rand::thread_rng;
use rand::Rng;

pub fn slice_rgen<T>(s: &[T]) -> &T {
    let idx = thread_rng().gen_range(0..s.len());

    unsafe { s.get_unchecked(idx) }
}
