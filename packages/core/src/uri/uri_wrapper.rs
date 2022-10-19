use super::uri::{ Uri };
use crate::wrapper::Wrapper;

pub struct UriWrapper<W: Wrapper> {
  pub uri: Uri,
  pub wrapper: W,
}
