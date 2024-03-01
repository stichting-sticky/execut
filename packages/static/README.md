<img
  src="./src/icons/logo-terminal.svg"
  alt="exec(ut)'s logo"
  width="128px"
  height="128px" />

# exec(ut)

Repository with the source code for the [execut.nl](https://execut.nl/) website. Build with Astro 🚀

## Getting started

```bash
$ git clone git@github.com:stichtingsticky/execut.git
```

### Commands

This project requires `pnpm` to be installed.

| Command        | Action                                                                       |
| :------------- | :--------------------------------------------------------------------------- |
| `pnpm install` | Installs dependencies                                                        |
| `pnpm start`   | Starts a local development server at [localhost:4321](https:localhost:4321/) |
| `pnpm build`   | Generates build artifacts at [`dist/`](./dist/)                              |
| `pnpm preview` | Previews the build locally                                                   |
| `pnpm check`   | Checks the project for errors                                                |
| `pnpm format`  | Runs `prettier`                                                              |

To ensure that dependencies are installed run `pnpm install` before any other command.

## Deployment

This site is hosted using GitHub Pages. All changes to the `main` branch will automatically be build and deployed.

In case of a failure the latest successful deployment will stay available.

## Content

Content of the site can be managed in [`src/content/`](./src/content/). 

**Ensure that after making changes the site still functions by running `pnpm build && pnpm preview` after making any changes in [`src/content/`](./src/content/).**
 
Alternately, to only check if the changes still match the collection schema `pnpm check` can be run. Please note that this does not check for any visual faults nor does it check if the site can still be build (and be deployed).

## License

Copyright 2023 - 2024 Sem van Nieuwenhuizen. All Rights Reserved. This project is licensed under the terms of the `MIT` license. You can check out the full license [here](./LICENSE).
