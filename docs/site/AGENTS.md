# Documentation site rules

Edit site content only under `docs/site`.

- Put shared templates in `_layouts`, reusable fragments in `_includes`, and navigation data in `_data/versions.yml`.
- Put current pages in `latest/` and release snapshots in `vX.Y/` (none exist yet — no tagged release has shipped with versioned documentation). Add every supported version to `_data/versions.yml`, including its `pages` list (slugs, used by the version selector's page-preserving fallback) and `nav` (sidebar entries).
- Start every rendered Markdown page with front matter containing `title`, `version`, and `slug` (the page's filename without extension; `index` for a version's landing page).
- Use ATX headings, sentence-case link labels, fenced code blocks with a language, and relative internal links filtered through `relative_url` in Liquid templates.
- Keep Liquid limited to layouts, includes, and the handful of `{{ '...' | relative_url }}` links pages use for cross-references. Do not put secrets, credentials, or private URLs in site content.
- Validate changes with `bundle install` and `bundle exec jekyll build` from this directory. Check the root, `/latest/`, and every version listed in `_data/versions.yml`.
- The Pages workflow (`.github/workflows/docs-pages.yaml`) publishes changes to `docs/site` from both `develop` (this repository's continuously-integrated branch) and `master` (which only moves on tagged releases). Do not edit a historical snapshot except to correct release documentation.
