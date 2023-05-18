#[macro_export]
macro_rules! resolver_vec {
  ($($resolver:expr),* $(,)?) => {
      Box::new(UriResolverAggregator::from(vec![$(Box::from($resolver) as Box<dyn UriResolver>),*])) as Box<dyn UriResolver>
  };
}
