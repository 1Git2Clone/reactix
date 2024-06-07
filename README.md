# Reactix

A monorepo consisting of a front-end (written in [React](https://react.dev/)
with [Tailwind CSS](https://tailwindcss.com/),
[TypeScript](https://www.typescriptlang.org/) and using
[Vite](https://vitejs.dev/)) and a back-end (written with
[Rust](https://www.rust-lang.org/) via the [Actix](https://actix.rs/)
framework).

## Purpose

This repository was meant to be kind of like a template for creating a
full-stack web application. I chose Actix because i wanted to give Rust
back-end programming another try (I didn't get very far with
[rocket.rs](https://rocket.rs/)) and React for the front-end because it's
widely used and it has native integration with a lot of the web development
tooling.

## Quick set up

### Back-end

- Go to [backend](backend/).

- Make a `.env` file.

- Write the following in it

```env
SSL_PASSWORD=your-password-here
```

- Run the back-end.

```sh
cargo run --release
```

### Front-end

- Go to [frontend](frontend/)

- Install dependencies

```sh
npm install
```

- Run the front-end

```sh
npm run watch # or npm run build && npm run preview
```

Now your content is displayed on: <https://127.0.0.1:42210/> (IPv4 localhost on
port `42210`).

## Module explanations

You can find details about the particular modules in their corresponding
`README.md` files.

- [back-end](backend/)

- [front-end](frontend/)
