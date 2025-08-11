# Tech Context

This document lists the technologies used, development setup, and technical constraints.

## Technologies

- Rust
- JavaScript
- React
- Vite
- Cargo

## Rust Best Practices

- Follow standard Rust idioms and conventions as outlined in "The Rust Book".
- Use `cargo clippy` for linting and identifying common mistakes.
- Use `rustfmt` for consistent code formatting.

## JavaScript Best Practices

- Follow a consistent style guide, such as the Airbnb JavaScript Style Guide.
- Use a linter like ESLint to catch common errors and enforce coding standards.
- Use a code formatter like Prettier for consistent code style.
- Prefer modern JavaScript (ES6+) features where appropriate.

## React Best Practices

- Maintain a good folder structure.
- Maintain a structured import order.
- Learn and use different component patterns.
- Use a linter and follow its rules.
- Test your code.
- Use TypeScript or at least `prop-types` and `default-props`.
- Use lazy-loading/code splitting.
- Extract reusable logic into custom hooks.
- Handle errors effectively.
- Keep the `key` prop unique.
- Use the `useReducer` hook for complex state.
- Use shorthand for boolean props.
- Avoid curly braces for string props.
- Erase non-HTML attributes when spreading props.
- Use snippet extensions with caution.
- Use fragments when a `div` is not needed.
- Use self-closing tags when no children are needed.
- Follow common naming conventions (PascalCase for components, camelCase for variables, etc.).
- Sanitize your code to prevent XSS attacks.

## Database Best Practices

- Keep the design simple and use standard naming conventions.
- Normalize data to minimize redundancy and protect data consistency.
- Design for the long term, create documentation, and plan for resource limitations.
- Foster collaboration between developers and database administrators.
- Model your data and create diagrams to ensure the design fits your needs.

## Development Setup

- TBD

## Technical Constraints

- TBD

## Committing

- Use conventional commits.
- One file per commit.
- Commit message should be a single line.
- No commit body.
