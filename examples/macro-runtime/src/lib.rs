use plugins_macro_rt::*;

on_load! {
	() {
		println!("`on_load!`: invocation #1");
		true
	}
}
on_load! {
	() {
		println!("`on_load!`: invocation #2");
		true
	}
}

on_all_loaded! {
	() {
		println!("`on_all_loaded!`: invoked");
	}
}

on_unload! {
	() {
		println!("`on_unload!`: invoked");
	}
}
