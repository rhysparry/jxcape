set windows-shell := ["nu", "-c"]

changelog:
    git cliff --output CHANGELOG.md
