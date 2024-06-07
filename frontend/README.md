# React + Tailwind + Vite + TS front-end

[![License Icon]][LICENSE]

[License Icon]: https://img.shields.io/badge/license-MIT-blue.svg
[LICENSE]: LICENSE

The front-end of this web app is _somewhat_ straight-forward.

This aims to be a simple configuration as a foundation for a modern front-end
on the web. It also has a simple API connection example. It's important that
both the front-end and the back-end communicate through the same SSL
configuration otherwise you'll get
[CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) issues. For
more information you can also refer to the [backend README.md
file](../backend/README.md).

The preview of this is shown in the `vite preview` command and it's in:
<https://127.0.0.1:42210/>

## File structure

The general [React
files](https://react.dev/learn/writing-markup-with-jsx#jsx-putting-markup-into-javascript)
are of either `.jsx` or `.tsx` extensions (one for
JavaScript and the other for TypeScript). The way it made is with functional
components that return HTML elements and can also process JavaScript logic.

The functions look like this:

```jsx
export default function App() {
  return (
    <>
      <h1>Hello world</h1>
      <p>
        Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint
        cillum sint consectetur cupidatat.
      </p>
    </>
  );
}
```

It has some extra rules like that `<>...</>` part is called a "fragment" tag.
It's used because everything needs to be under one tag in jsx/tsx. It doesn't
affect the DOM rendering in any way though (doesn't make an extra empty tag or
anything like that).

### [package.json](package.json)

The file containing all the required dependencies. It also contains cool stuff
like scripts that you can make and use with yarn/npm.

### [package-lock.json](package-lock.json)

The file containing locked (frozen) versions of the dependencies. Good for
debugging in some cases.

### [yarn.lock](yarn.lock)

Has the same purpose as [package-lock.json](./package-lock.json) but for the
yarn package manager (which is used because it's a lot quicker than npm).

### [tsconfig.json](tsconfig.json)

Used for TypeScript compiler options, mostly left with the default vite setup.

### [tsconfig.node.json](tsconfig.node.json)

As the name implies it's for TypeScript again, however, this file is for NodeJS
environments only. This means its primarily for the development environment.

### [postcss.config.js](postcss.config.js)

It contains JavaScript-based plugins which are used for CSS operations. The
most popular one being [TailwindCSS](https://tailwindcss.com/).

### [tailwind.config.js](tailwind.config.js)

TailwindCSS configurations such as the affected file extensions and also
purging options (handy for release mode).

### [vite.config.js](vite.config.js)

[Vite](https://vitejs.dev/) consists of front-end tools such as a preview and
build command but also options for [quickstarting your
projects](https://vitejs.dev/guide/#scaffolding-your-first-vite-project). The
[vite.config.js](vite.config.ts) file in particular is used for configuring
your `vite build`.

## Setting up

The setup process is rather simple. I recommend reading in the [package.json
file](package.json) to get to know more about the scripts that get ran. It uses
`nodemon` for automatic recompiling.

### npm

```sh
npm install
npm run watch # or: npm run build && npm preview
```

---

### yarn

If you don't already have yarn installed (you may need to do the command with
`sudo` because its `--global`).

```sh
npm install yarn --global
```

Proceed as normal.

```sh
yarn install
yarn run watch # or: yarn run build && yarn preview
```
