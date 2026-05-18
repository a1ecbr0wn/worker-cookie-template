# cookie worker

A cookie consent banner for a site, powered by [worker-cookie](https://github.com/a1ecbr0wn/worker-cookie).

## Setup

1. Edit `wrangler.jsonc` — change `name` to your worker name.
2. Edit `config/cookie-banner.toml` — set your message, theme, and scripts.
3. Add a `CLOUDFLARE_API_TOKEN` secret to this repository (Settings → Secrets → Actions).
4. Push — GitHub Actions will build and upload the worker.

See the [worker-cookie documentation](https://cookies.a1ecbr0wn.com/) for
configuration reference and theme options.

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
