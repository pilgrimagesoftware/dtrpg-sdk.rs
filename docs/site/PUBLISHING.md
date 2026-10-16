# Publishing

Pages source is set to "GitHub Actions" (Settings > Pages > Build and deployment > Source), and the `github-pages` environment allows deployments from both `develop` and `master`.

Pushes to `develop` or `master` that change `docs/site` or `.github/workflows/docs-pages.yaml` build and deploy the site.

Before merging, run `bundle install` and `bundle exec jekyll build` from `docs/site`. Confirm the generated root redirects to `latest`, and open each path listed in `_data/versions.yml`.

After a successful deployment, verify the published root URL, `/latest/`, and the version selector (even with one entry) resolve correctly.
