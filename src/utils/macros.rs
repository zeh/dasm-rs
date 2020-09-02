#[macro_export]
macro_rules! log_function {
	() => {{
		fn f() {}
		fn type_name_of<T>(_: T) -> &'static str {
			std::any::type_name::<T>()
			}
		let name = type_name_of(f);
		println!("[FFF] {}()", &name[..name.len() - 3]);
		// Can also use: module_path!(), file!(), line!(), column!()
		}};
}

#[macro_export]
macro_rules! log_function_with {
	($($arg:tt)*) => {{
		fn f() {}
		fn type_name_of<T>(_: T) -> &'static str {
			std::any::type_name::<T>()
			}
		let name = type_name_of(f);
		println!("[FFF] {}() :: {}", &name[..name.len() - 3], format!($($arg)*));
		// Can also use: module_path!(), file!(), line!(), column!()
		}};
}
