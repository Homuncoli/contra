cargo release version minor --workspace --execute --no-confirm
git add .
git commit -m "chore: update version"
cargo release --workspace --dependent-version upgrade --execute --no-confirm