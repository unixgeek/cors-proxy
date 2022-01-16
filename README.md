# CORS Proxy
cors-proxy is a Cloudflare Worker for proxying CORS requests to resources that do not support CORS. 
It may also work with resources that do support CORS, but not for your origin. There are probably
better, more technically correct solutions available -- this was a project to learn about Cloudflare
and to be used with other pet projects.
## Prerequisites
1. Node.js
2. @cloudflare/wrangler
3. miniflare

    [Miniflare](https://miniflare.dev) is optional for development. It is a Cloudflare Workers
      simulator. It is useful as it allows your worker to run on your local machine as opposed to
      deploying to Cloudflare with wrangler dev. With wrangler dev, your worker cannot talk to resources
      running on your local machine, for example.
5. Rust
6. Rust wasm32-unknown-unknown target
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
## wrangler
To use wrangler you will need to create a preview version of the namespace and replace `kv_namespaces` in `wrangler.toml`
with the output.  

    wrangler kv:namespace create --preview default

And a preview version of the `allowed_hostnames` key. Replace `ID` with the id from the previous step.

    wrangler kv:key put allowed_hostnames "localhost,some.example.com" --namespace-id REPLACE_WITH_PREVIEW_ID

Finally, run `wrangler dev` and it will build the project and allow you to preview it "locally". Remember that it is 
running on the Cloudflare servers and not your local machine. 

## miniflare
To use miniflare, you will need to do an additional step: update the file in `.mf/kv/default/allowed_hostnames` with
the list of hostnames. Then start with `miniflare --kv default --kv-persist`. 

# Resources
[Cloudflare Workers](https://developers.cloudflare.com/workers/)  
[Cloudflare worker crate](https://crates.io/crates/worker)  
[Miniflare](https://miniflare.dev)