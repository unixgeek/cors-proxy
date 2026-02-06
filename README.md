# CORS Proxy
cors-proxy is a Cloudflare Worker for proxying CORS requests to resources that do not support CORS. 
It may also work with resources that do support CORS, but not for your origin. There are probably
better, more technically correct solutions available -- this was a project to learn about Cloudflare
and to be used with other pet projects.

[![Deploy to Cloudflare Workers](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/unixgeek/cors-proxy)

## Requirements
* [Rust](https://www.rust-lang.org)
* [wasm-pack](https://crates.io/crates/wasm-pack)
* [worker-build](https://crates.io/crates/worker-build)
* [Node.js](https://nodejs.org/en/)

## Development Setup
 Install dependencies.

       npm install
### Running Locally
1. Create the required KV data store and allow requests from localhost. `allowed_hostnames` allows for multiples hosts,
separated by a comma.

       npx wrangler --env dev kv:key put allowed_hostnames localhost --binding default --preview true --local
2. Run the local server.

       npx wrangler --env dev dev

### Running in Cloudflare Preview
1. Create a preview version of the `default` namespace in your Cloudflare account.

       npx wrangler kv:namespace create --preview true default

2. Take the id provided and replace `preview_id` of `kv_namespaces` in `wrangler.toml`.
3. Add a preview version of the `allowed_hostnames` key.

       npx wrangler kv:key put --binding default --preview true allowed_hostnames "localhost,some.example.com"

4. Run the remote server.

       npx wrangler dev --remote # --preview true?

## Deployment
1. Create the `default` namespace in your Cloudflare account.

       npx wrangler kv:namespace create default
2. Take the id provided and replace `id` of `kv_namespaces` in `wrangler.toml`.
3. Add the `allowed_hostnames` key. This is a comma separated list of hostnames that are allowed to use the cors-proxy worker.

       npx wrangler kv:key put --binding default allowed_hostnames "localhost,some.example.com"
4. Deploy to Cloudflare.

       npx wrangler deploy

## Resources
[Cloudflare Workers](https://developers.cloudflare.com/workers/)  
[Cloudflare worker crate](https://crates.io/crates/worker)  


Read the docs and figure out env / preview stuff.
Create simple http client to confirm header behavior.
Need to copy parameters.
Rename url to cors-proxy-url.



https://github.com/cloudflare/workers-sdk
https://github.com/cloudflare/wrangler-action
https://github.com/cloudflare/workerd/blob/8cb7ffdec8477ea73d9d1df9eceb2ff4986decd6/types/generated-snapshot/oldest/index.ts#L4107
https://developers.cloudflare.com/fundamentals/reference/http-request-headers/#accept-encoding
https://github.com/cloudflare/workers-sdk/issues/5246
https://github.com/cloudflare/workers-sdk/pull/5409#issuecomment-2022576116

