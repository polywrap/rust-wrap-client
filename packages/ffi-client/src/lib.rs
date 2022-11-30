use std::{ffi::CStr, sync::Arc};

use filesystem_plugin::FileSystemPlugin;
use fs_resolver_plugin::FileSystemResolverPlugin;
use futures::{lock::Mutex, executor::block_on};
use libc::c_char;
use polywrap_client::polywrap_client::PolywrapClient;
use polywrap_core::{
    client::ClientConfig,
    invoke::{Invoker, InvokeArgs},
    resolvers::{
        static_resolver::{StaticResolver, StaticResolverLike},
        uri_resolution_context::UriPackage,
    },
    uri::Uri,
};
use polywrap_plugin::package::PluginPackage;

macro_rules! create_free_fn {
    ($entity:ty, $name: ident) => {
        #[no_mangle]
        pub extern "C" fn $name(ptr: *mut $entity) {
            if ptr.is_null() {
                return;
            }
            unsafe {
                drop(Box::from_raw(ptr));
            }
        }
    };
}

#[no_mangle]
pub extern "C" fn create_static_resolver() -> *mut StaticResolver {
    let fs = FileSystemPlugin {};
    let fs_plugin_package: PluginPackage = fs.into();
    let fs_package = Arc::new(Mutex::new(fs_plugin_package));

    let fs_resolver = FileSystemResolverPlugin {};
    let fs_resolver_plugin_package: PluginPackage = fs_resolver.into();
    let fs_resolver_package = Arc::new(Mutex::new(fs_resolver_plugin_package));

    let resolver = StaticResolver::from(vec![
        StaticResolverLike::Package(UriPackage {
            uri: Uri::try_from("wrap://ens/fs.polywrap.eth").unwrap(),
            package: fs_package,
        }),
        StaticResolverLike::Package(UriPackage {
            uri: Uri::try_from("wrap://ens/fs-resolver.polywrap.eth").unwrap(),
            package: fs_resolver_package,
        }),
    ]);

    Box::into_raw(Box::new(resolver))
}

create_free_fn!(StaticResolver, static_resolver);

#[no_mangle]
pub extern "C" fn create_client(resolver: *mut StaticResolver) -> *mut PolywrapClient {
    let resolver = unsafe {
        assert!(!resolver.is_null());
        Arc::from_raw(resolver)
    };

    let client = PolywrapClient::new(ClientConfig {
        envs: None,
        interfaces: None,
        resolver,
    });

    Box::into_raw(Box::new(client))
}

create_free_fn!(PolywrapClient, polywrap_client);

#[no_mangle]
pub extern "C" fn invoke(
    client: *mut PolywrapClient,
    uri: *const c_char,
    method: *const c_char,
    args: *mut u8,
    args_len: libc::size_t,
) -> *mut u32 {
    let client = unsafe {
        assert!(!client.is_null());
        Box::from_raw(client)
    };

    let uri = unsafe {
        assert!(!uri.is_null());

        CStr::from_ptr(uri)
    }
    .to_str()
    .unwrap().to_string();

    let method = unsafe {
        assert!(!method.is_null());

        CStr::from_ptr(method)
    }
    .to_str()
    .unwrap();

    let args = unsafe {
        let len = args_len as usize;
        Vec::from_raw_parts(args, len, len)
    };

    let uri: Uri = uri.try_into().unwrap(); 

    let invoke_result = block_on(async {
      client.invoke(&uri, method, Some(&InvokeArgs::UIntArray(args)), None, None).await.unwrap()
    });

    Box::into_raw(Box::new(invoke_result)) as *mut _
}
