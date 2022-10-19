use super::uri::{ Uri };
use crate::wrapper::Wrapper;

struct UriWrapper<W: Wrapper> {
  uri: Uri,
  wrapper: W,
}
