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

       npx wrangler --env dev kv:key put allowed_hostnames localhost --binding default --local
2. Run the local server.

       npx wrangler --env dev dev

### Running in Cloudflare Preview
1. Create a preview version of the `default` namespace in your Cloudflare account.

       npx wrangler kv:namespace create --preview true default

2. Take the id provided and replace `preview_id` of `kv_namespaces` under `[env.prod]` in `wrangler.toml`.
3. Add a preview version of the `allowed_hostnames` key.

       npx wrangler kv:key put --binding default --preview true allowed_hostnames "localhost,some.example.com"

4. Run the remote server.

       npx wrangler dev --remote

## Deployment
1. Create the `default` namespace in your Cloudflare account.

       npx wrangler kv:namespace create default
2. Take the id provided and replace `id` of `kv_namespaces` under `[env.prod]` in `wrangler.toml`.
3. Add the `allowed_hostnames` key. This is a comma separated list of hostnames that are allowed to use the cors-proxy worker.

       npx wrangler kv:key put --binding default allowed_hostnames "localhost,some.example.com"
4. Deploy to Cloudflare.

       npx wrangler --env prod deploy

## Resources
[Cloudflare Workers](https://developers.cloudflare.com/workers/)  
[Cloudflare worker crate](https://crates.io/crates/worker)  
[Miniflare](https://miniflare.dev)
