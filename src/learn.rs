trait Learn<T> {
	pub train(inp: &[T]);
	pub infer(inp: &[T]);
}
