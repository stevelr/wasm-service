use crate::{handler_return, Context, Error, Handler, HandlerReturn, HttpDate, Method, Request};
use async_trait::async_trait;
//use service_logging::{log, Severity};
use std::str::FromStr;

use kv_assets::{AssetMetadata, KVAssets};

/// Serves static assets out of Worker KV storage.
pub struct StaticAssetHandler<'assets> {
    kv: KVAssets<'assets>,
}

impl<'assets> StaticAssetHandler<'assets> {
    /// Initialize static asset handler
    /// `index_bin` is the serialized AssetIndex, which will be deserialized lazily (if needed)
    /// `account_id` is Cloudflare account id
    /// `namespace_id` is cloudflare KV namespace id (the long hex string, not the friendly name)
    /// `auth_token` Cloudflare api OAuth token
    pub fn init(
        index_bin: &'assets [u8],
        account_id: &'_ str,
        namespace_id: &'_ str,
        auth_token: &'_ str,
    ) -> Self {
        Self {
            kv: KVAssets::init(index_bin, account_id, namespace_id, auth_token),
        }
    }

    /// Returns true if there is a static asset matching this path.
    /// Only checks the manifest - does not check KV. This could give a false positive
    /// positive if the manifest is out of date, so site developers must ensure that
    /// the manifest is regenerated and pushed if any user deletes static content from KV.
    /// That is unlikely to occur if kv-sync is being used to update the manifest
    /// and values in the static namespace at the same time.
    ///
    /// There is also a potential scenario where this function could return true and the handler
    /// later has a network problem reading from KV, or the account credentials are bad,
    /// and the end user isn't able to retrieve content for which this method returns true.
    ///
    /// Due to these two potential problems, a true result isn't a 100% guarantee that
    /// the user will receive content, but in the presence of good deploy practices
    /// and reliable networking, this should be accurate.
    pub fn has_asset(&self, req: &Request) -> bool {
        (req.method() == Method::GET || req.method() == Method::HEAD)
            && matches!(self.kv.lookup_key(req.url().path()), Ok(Some(_)))
    }

    /// Does some quick checks and may return
    /// - 304 Not Modified, if request had if-modified-since header and doc was <= header date
    /// - 200 if request was HEAD method
    /// Returns Ok(None) if content is not found (no path match)
    /// Returns Ok(Some(metadata)) if doc is found
    fn check_metadata(
        &self,
        path: &str,
        req: &Request,
        ctx: &mut Context,
    ) -> Result<Option<AssetMetadata>, HandlerReturn> {
        use reqwest::header::IF_MODIFIED_SINCE;

        match self.kv.lookup_key(path) {
            Err(e) => {
                ctx.raise_internal_error(Box::new(e));
                Err(handler_return(200, "")) // handle internal error higher in the stack
            }
            Ok(None) => {
                // file not found
                Ok(None)
            }
            Ok(Some(md)) => {
                // GET or HEAD
                if let Some(dt) = req.get_header(IF_MODIFIED_SINCE.as_str()) {
                    if let Ok(http_date) = HttpDate::from_str(dt.as_str()) {
                        // valid if-modified-since header with parsable date
                        // if kv is same or older (smaller time), return Not Modified
                        if md.modified <= http_date.timestamp() as u64 {
                            return Err(handler_return(304, "Not Modified"));
                        }
                        // else modified, so fall through
                    } else {
                        // don't bother logging date parse errors
                        //log!(ctx, Severity::Warning, _:"parse_date_err", val:&dt)
                    }
                }
                /*
                // HEAD only
                if req.method() == Method::HEAD {
                    ctx.response()
                        .header(LAST_MODIFIED, HttpDate::from(md.modified).to_string())
                        .unwrap(); // unwrap is ok because number.to_string() is always ascii
                    Err(handler_return(200, ""))
                } else {
                    Ok(md)
                }
                */
                Ok(Some(md))
            }
        }
    }
}

fn remove_leading_slash(path: &str) -> &str {
    path.strip_prefix('/').unwrap_or(path)
}

#[async_trait(?Send)]
impl<'assets> Handler for StaticAssetHandler<'assets> {
    /// Process incoming Request. If no asset was found at the request path, response.is_unset() will be true.
    /// Only handles GET and HEAD requests.
    async fn handle(&self, req: &Request, mut ctx: &mut Context) -> Result<(), HandlerReturn> {
        let path = remove_leading_slash(req.url().path());
        if (req.method() != Method::GET && req.method() != Method::HEAD) || path.is_empty() {
            return Ok(());
        }
        // change trailing slash, indicating 'folder' to folder/index.html
        let path = if path.ends_with('/') {
            format!("{}/index.html", path)
        } else {
            path.to_string()
        };
        // This may return quickly if response can be satisfied without querying KV,
        // such as HEAD requests, or If-modified-since header when it hasn't been modified
        let md = match self.check_metadata(&path, req, &mut ctx)? {
            None => return Ok(()), // not found: fall through to let service handler deal with it
            Some(md) => md,
        };
        // have metadata, asset is in KV (unless manifest is out of date)
        match self.kv.get_kv_value(&md.path).await {
            Ok(bytes) => {
                // if we can figure out the content type, report it
                // otherwise let browser sniff it
                if let Some(mt) = crate::media_type(&md.path) {
                    ctx.response()
                        .header(reqwest::header::CONTENT_TYPE, mt.to_string())
                        .unwrap();
                }
                ctx.response()
                    .header("last-modified", HttpDate::from(md.modified).to_string())
                    .unwrap()
                    .body(bytes.to_vec());
            }
            Err(e) => {
                ctx.raise_internal_error(Box::new(Error::Other(format!(
                    "static asset lookup failed path({}) url-path({}) error:{}",
                    &md.path,
                    path,
                    e.to_string()
                ))));
            }
        }
        Ok(())
    }
}
