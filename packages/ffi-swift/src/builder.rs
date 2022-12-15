use std::{collections::HashMap, ffi::CStr};

use polywrap_core::{resolvers::{static_resolver::{StaticResolver, StaticResolverLike}, recursive_resolver::RecursiveResolver, uri_resolver_like::UriResolverLike}, client::UriRedirect};

pub struct ResolverBuilder {
    statics: Vec<StaticResolverLike>,
}

impl ResolverBuilder {
    pub fn new() -> ResolverBuilder {
        ResolverBuilder { statics: vec![] }
    }

    pub fn add_static(&mut self, resolver: Box<StaticResolverLike>, redirects: HashMap<String, String>) {
        self.statics.push(*resolver);
    }

    pub fn build(self) -> RecursiveResolver {
        let static_resolver = Box::new(
            StaticResolver::from(self.statics)
        );
        RecursiveResolver::from(vec![
            UriResolverLike::Resolver(static_resolver)
        ])
    }
}


#[no_mangle]
pub extern "C" fn create_builder() -> *const libc::c_char {
    Box::into_raw(Box::new(ResolverBuilder::new())) as *const libc::c_char
}

#[no_mangle]
pub extern "C" fn create_static_resolver(from: *const libc::c_char, to: *const libc::c_char) -> *const libc::c_char {
    let from_c_str = unsafe { CStr::from_ptr(from) };
    let from_str = match from_c_str.to_str() {
        Ok(u) => u,
        Err(_) => panic!("Couldn't get CStr for from")
    };

    let to_c_str = unsafe { CStr::from_ptr(to) };
    let to_str = match to_c_str.to_str() {
        Ok(u) => u,
        Err(_) => panic!("Couldn't get CStr for to")
    };

    let redirect = UriRedirect::new(from_str.try_into().unwrap(), to_str.try_into().unwrap());

    let redirects_static_like = StaticResolverLike::Redirect(redirect);
    let static_resolver = StaticResolver::from(vec![redirects_static_like]);
    Box::into_raw(Box::new(static_resolver)) as *const libc::c_char
}

#[no_mangle]
pub extern "C" fn add_static_resolver(builder: *const libc::c_char, resolver: *const libc::c_char) {
    let mut b: Box<ResolverBuilder> = unsafe { Box::from_raw(builder as *mut ResolverBuilder) };
    let r: Box<StaticResolverLike> = unsafe { Box::from_raw(resolver as *mut StaticResolverLike) };
    b.add_static(r, HashMap::new());

}

#[no_mangle]
pub extern "C" fn build_resolver(builder: *const libc::c_char) -> *const libc::c_char {
    let b: Box<ResolverBuilder> = unsafe { Box::from_raw(builder as *mut ResolverBuilder) };
    Box::into_raw(Box::new(b.build())) as *const libc::c_char
}
