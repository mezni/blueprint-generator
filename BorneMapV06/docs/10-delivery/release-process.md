# Release Process

## Versioning

- Semantic versioning (MAJOR.MINOR.PATCH)
- All services versioned together in monorepo
- Git tags used for releases

## Release Steps

1. **Code Complete** — all PRs merged to main
2. **CI Passes** — all workflows green on main
3. **Tag Release** — `git tag v1.0.0 && git push origin v1.0.0`
4. **Build Images** — CI builds Docker images on tag
5. **Manual Deploy** — operator pulls images and deploys to production
6. **Smoke Test** — verify key flows work in production
7. **Announce** — inform stakeholders of release

## Hotfix Process

1. Branch from tag
2. Apply fix
3. Tag hotfix version (v1.0.1)
4. Build + deploy
5. Merge fix back to main
