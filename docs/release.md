# Release

This file is a reminder for integrators on how to release a new fil version.

First of all, only a fil integrator is allowed to release a new version.

Note: in all following examples, replace `vX.Y.Z` by the released version.

## Go on top of master branch

```shell
git checkout master
git fetch origin
git pull
```

## Update the changelog

In [CHANGELOG.md](../CHANGELOG.md), replace the `[Unreleased]` header by `vX.Y.Z` and create a new `[Unreleased]` section above.

## Contributors list

Update the contributors list with `generate-contributors-list`. If there is new contributors, note who they are and what is their first contribution.

## Commit and tag

```shell
git commit -S -am "chore(release): fil vX.Y.Z"
git tag -s -m "fil vX.Y.Z" vX.Y.Z
git push origin
git push --tags origin
```

## Create the release on GitHub

Create the release with tag `vX.Y.Z`, copy the release note from [CHANGELOG.md](../CHANGELOG.md). If needed add a section about new contributors with a link to their first contribution. Check the set as latest release checkbox.

We're done!
