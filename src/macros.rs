#[macro_export]
macro_rules! combine {
	($($mode:expr),+ ; $($sep: expr),*) => {
		$crate::Rg::new().combine::<&str, _>(
			&[$(&$mode),+],
			&[$($sep),*]
		)
	};
}
