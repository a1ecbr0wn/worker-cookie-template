use worker::*;

/// Cloudflare Worker fetch entry point for the cookie consent banner.
///
/// Routes incoming requests to the cookie consent banner middleware via `worker_cookie::run()`.
/// See the `worker_cookie` crate documentation for behavior details.
#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    worker_cookie::run(req, env, ctx).await
}
