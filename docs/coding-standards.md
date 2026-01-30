# Coding standards

Following guidelines are not here to bother you but to keep fil code quality high.

## Basic principles

- Test your code, follow [TDD](https://en.wikipedia.org/wiki/Test-driven_development)
- [SOLID](https://en.wikipedia.org/wiki/SOLID)
- [YAGNI](https://en.wikipedia.org/wiki/You_aren't_gonna_need_it)
- [DRY](https://en.wikipedia.org/wiki/Don't_repeat_yourself)

To be extensive, you can even take a look at [Software Craft](https://en.wikipedia.org/wiki/Software_craftsmanship).

Write code easy to read. If you need to comment each line, it means your code is not comprehensible. A good code doesn't need to be commented.

## Commit

Please follow [Conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) guidelines for your commit message. A [template](../commit-template) is available and configured automatically in your local git config, see next section for more details. 

Your commit MUST work. If your feature is too big to be in one commit, add exceptions, early returns, feature flag to keep the code working.

In your commit message, reference the id of your issue. It helps us to track which commit concern what and why.

In the description, don't hesitate to be extensive on why. It will help the reviewer to understand your reasoning.

## Tools

All tools are provided within a [nix](https://nixos.org/) flake dev shell. So you just need nix and enable flakes.

This dev shell add in your `PATH` environment some executable in [tools/bin](../tools/bin) and also configure your local git config.

## Tests

If you submit a bug fix, add a test case reproducing the bug. If it's a new feature, add tests covering it.

## License

By contributing to fil, you agree that your contributions will be licensed under the [GLP-2.0-or-later](../LICENSE.txt) license.

Each code file MUST begin with the following snippet:

```
/**
 * fil
 * Copyright (C) <year> - Present  fil contributors
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */
```
