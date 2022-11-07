use crate::uri_resolution_context::{ 
    UriPackageOrWrapper, UriWrapper, UriPackage
};
use crate::error::Error;

struct UriResolutionResult {
    result: Result<UriPackageOrWrapper, Error>
}

enum PackageOrWrapper {
    Package(WrapPackage),
    Wrapper(Wrapper)
}

impl UriResolutionResult {
    pub fn ok(
        uri: &Uri, 
        package_or_wrapper: Option<PackageOrWrapper>
    ) -> Result<UriPackageOrWrapper, Error> {
        if let Some(i) = package_or_wrapper {
            match i {
                Wrapper(w) => {
                    return Ok(UriWrapper {
                        uri,
                        wrapper: w
                    });
                },
                Package(p) => {
                    return Ok(UriPackage {
                        uri,
                        package: p
                    });
                }
            }
        }

        return Ok(uri)
    }
}