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
## Initial Setup and Deployment
The following steps are needed to deploy cors-proxy to Cloudflare.
1. Install dependencies.

       npm install
2. Login to Cloudflare

       npx wrangler login
3. Create a namespace and replace the current `kv_namespaces` in `wrangler.toml` with the output.

       npx wrangler kv:namespace create default
4. Create `allowed_hostnames` key. This key is a comma separated list of hostnames that are allowed to use the cors-proxy worker. 

       npx wrangler kv:key put --preview false --binding default allowed_hostnames "localhost,some.example.com"
5. Publish to Cloudflare.

       npx wrangler publish
## Development Setup
    npm install
Optionally swap the `command` under `[build]` in `wrangler.toml` for better debugging.
### Wrangler
You will need to create a preview version of the namespace and replace `preview_id` of `kv_namespaces` in `wrangler.toml`
with the output.  

    npx wrangler kv:namespace create --preview default

Add a preview version of the `allowed_hostnames` key.

    npx wrangler kv:key put --binding default --preview allowed_hostnames "localhost,some.example.com"

Run `npx wrangler dev` and it will build the project and allow you to preview it "locally". Remember that it is 
running on the Cloudflare servers and not your local machine. 

### Miniflare
Miniflare will run the worker on your local machine and it is built in to wrangler. Namespaces work differently with
Miniflare. To update the `allowed_hostnames` key, modify `.wrangler/state/kv/default/allowed_hostnames`.

Then run miniflare.

    npx wrangler dev --local --persist

## Docker
A Docker image that hosts cors-proxy with miniflare is available.

    docker container run --publish 8787:8787 --rm -i -t unixgeek2/cors-proxy

This is useful when working on an application that uses cors-proxy as it will save requests to your production deployed
worker. It probably isn't useful outside that use case.

## Resources
[Cloudflare Workers](https://developers.cloudflare.com/workers/)  
[Cloudflare worker crate](https://crates.io/crates/worker)  
[Miniflare](https://miniflare.dev)
