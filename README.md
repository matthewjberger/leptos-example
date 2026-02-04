# Leptos Template

A minimal Leptos template with Tailwind CSS, dark mode support, and client-side rendering.

## Features

- Leptos 0.8 with CSR (client-side rendering)
- Tailwind CSS for styling
- Dark mode with localStorage persistence
- Responsive navigation with mobile menu
- Trunk for WASM bundling

## Prerequisites

- [Rust](https://rustup.rs/) (nightly)
- [Trunk](https://trunkrs.dev/) (`cargo install trunk`)
- [Node.js](https://nodejs.org/) and npm
- [just](https://github.com/casey/just) (optional, for task running)
- [leptosfmt](https://github.com/bram209/leptosfmt) (optional, for formatting)

## Getting Started

```bash
# Install dependencies
npm install

# Build Tailwind CSS and start dev server
just serve

# Or manually:
npx tailwindcss -i ./input.css -o ./public/styles.css --minify
trunk serve --port 3000 --open
```

## Commands

| Command | Description |
|---------|-------------|
| `just serve` | Build Tailwind and start dev server |
| `just build` | Build for production |
| `just check` | Run cargo check and format check |
| `just lint` | Run clippy |
| `just format` | Format code |
| `just tailwind-watch` | Watch Tailwind for changes |

## Project Structure

```
src/
├── main.rs          # Entry point
├── lib.rs           # App component with routing and theme
├── pages.rs         # Page module declarations
├── pages/
│   ├── home.rs      # Home page
│   └── not_found.rs # 404 page
├── components.rs    # Component module declarations
└── components/
    └── navigation.rs # Navigation bar
```

## Customization

1. Update the name in `src/components/navigation.rs`
2. Update the title in `src/lib.rs`
3. Modify the hero content in `src/pages/home.rs`
4. Add new pages in `src/pages/` and register routes in `src/lib.rs`
5. Add new components in `src/components/`

## License

MIT
