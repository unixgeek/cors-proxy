# CORS Proxy
cors-proxy is a Cloudflare Worker for proxying CORS requests to resources that do not support CORS. 
It may also work with resources that do support CORS, but not for your origin. There are probably
better, more technically correct solutions available -- this was a project to learn about Cloudflare
and to be used with other pet projects.

[![Deploy to Cloudflare Workers](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/unixgeek/cors-proxy)
## Requirements
* [Rust](https://www.rust-lang.org)
* [wasm-pack](https://crates.io/crates/wasm-pack)
* Cloudflare's build tool [worker-build](https://crates.io/crates/worker-build)
* [Node.js](https://nodejs.org/en/)
## Initial Setup and Deployment
The following steps are needed to deploy cors-proxy to Cloudflare.
1. Update `account_id` in `wrangler.toml` with your account id.
2. Create a namespace and replace the current `kv_namespaces` in `wrangler.toml` with the output.

       wrangler kv:namespace create default
3. Create `allowed_hostnames` key. This key is a comma separated list of hostnames that are allowed to use the cors-proxy worker. 

       wrangler kv:key put allowed_hostnames "localhost,some.example.com" --binding default
4. Publish to Cloudflare.

       wrangler publish
## Development Setup
    npm install
Optionally swap the `command` under `[build]` in `wrangler.toml` for better debugging.
### wrangler
To use wrangler you will need to create a preview version of the namespace and replace `kv_namespaces` in `wrangler.toml`
with the output.  

    npx wrangler kv:namespace create --preview default

And a preview version of the `allowed_hostnames` key. Replace `ID` with the id from the previous step.

    npx wrangler kv:key put allowed_hostnames "localhost,some.example.com" --namespace-id ID

Finally, run `npx wrangler dev` and it will build the project and allow you to preview it "locally". Remember that it is 
running on the Cloudflare servers and not your local machine. 

For additional debugging and profiling, run `npx wrangler dev --inspect`.

### miniflare
To update the `allowed_hostnames` key when using miniflare, modify `.mf/kv/default/allowed_hostnames`.

Then run miniflare.

       npx miniflare --kv default --kv-persist 

## Docker
A Docker image that hosts cors-proxy with miniflare is available.

    docker pull unixgeek2/cors-proxy

This is useful when working on an application that uses cors-proxy as it will save requests to your production deployed
worker. It probably isn't useful outside that use case.

## Resources
[Cloudflare Workers](https://developers.cloudflare.com/workers/)  
[Cloudflare worker crate](https://crates.io/crates/worker)  
[Miniflare](https://miniflare.dev)

TODO github action for deploy button and automatic build & deploy of docker container and worker.
https://docs.github.com/en/actions/learn-github-actions/understanding-github-actions