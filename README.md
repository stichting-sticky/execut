![exec(ut) logo](./icon.svg)

# exec(ut)

Repository with the source code for the [execut.nl](https://execut.nl/) website. Build with Astro 🚀

## Getting started

```bash
$ git clone git@github.com:stichting-sticky/execut.git
```

### Commands

This project requires `bun` to be installed 🥟

| Command           | Action                                                                       |
| :---------------- | :--------------------------------------------------------------------------- |
| `bun install`     | Installs dependencies                                                        |
| `bun run start`   | Starts a local development server at [localhost:4321](https:localhost:4321/) |
| `bun run build`   | Generates build artifacts at [`dist/`](./dist/)                              |
| `bun run preview` | Previews the build locally                                                   |
| `bun run check`   | Checks the project for errors                                                |

To ensure that dependencies are installed run `bun install` before any other command.

## Deployment

This site is hosted using GitHub Pages. All changes to the `main` branch will automatically be build and deployed.

In case of a failure the latest successful deployment will stay available.

## Content

Content of the site can be managed in [`src/content/`](./src/content/).

**Ensure that after making changes the site still functions by running `bun run build && bun run preview` after making any changes in [`src/content/`](./src/content/).**

Alternately, to only check if the changes still match the collection schema `bun run check` can be run. Please note that this does not check for any visual faults nor does it check if the site can still be build (and be deployed).

## License

Copyright (c) 2023 - 2025 Sem van Nieuwenhuizen. All Rights Reserved. This project is licensed under the terms of the MIT license. You can check out the full license [here](./LICENSE).
