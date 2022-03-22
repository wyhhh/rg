use rand::thread_rng;
use rand::Rng;
use rand::distributions::uniform::SampleUniform;
use rand::distributions::uniform::SampleRange;

pub fn rand_slice<T>(s: &[T]) -> &T {
    let idx = thread_rng().gen_range(0..s.len());

    unsafe { s.get_unchecked(idx) }
}

pub fn rand_or() -> bool {
	thread_rng().gen()
}

pub fn rand_range<T, R>(range: R) -> T
where
	T: SampleUniform,
	R: SampleRange<T>
{
	thread_rng().gen_range(range)
}
