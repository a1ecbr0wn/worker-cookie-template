# cookie worker template

This is a template to help you add a Cloudflare worker that provides a cookie
consent banner for a site, powered by [cookie-worker](https://github.com/a1ecbr0wn/worker-cookie).

## Setup

1. See the [cookie-worker documentation](https://cookies.a1ecbr0wn.com/setup) for
   how to create your repository from this template, configuration reference and
   theme options.
2. Edit `wrangler.jsonc` — change `name` to your worker name.
3. Edit `config/cookie-banner.toml` — set your message, theme, and scripts.
4. Add a `CLOUDFLARE_API_TOKEN` secret to this repository (Settings → Secrets →
   Actions).
5. Push — GitHub Actions will build and upload the worker.

See the [cookie-worker documentation](https://cookies.a1ecbr0wn.com/configuration)
for configuration reference and theme options.

## Local development

```sh
wrangler dev --var "WORKER_CONFIG:$(cat config/cookie-banner.toml)"
```

## Deploying a version

Each push uploads a new draft version. To make a version live:

```sh
wrangler versions deploy --yes
```

To cut a tagged release, trigger the **Tag a release** workflow from the Actions
tab.
