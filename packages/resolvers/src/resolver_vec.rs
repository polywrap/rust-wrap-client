#[macro_export]
macro_rules! resolver_vec {
  ($($resolver:expr),* $(,)?) => {
      vec![$(Box::from($resolver) as Box<dyn UriResolver>),*] as Vec<Box<dyn UriResolver>>
  };
}
