FROM mcr.microsoft.com/vscode/devcontainers/rust:1-bullseye

# Install Trunk CLI
RUN cargo install trunk

# Add WASM target
RUN rustup target add wasm32-unknown-unknown

# Install Tailwind CSS
RUN apt update
RUN curl -sL https://deb.nodesource.com/setup_16.x | sudo bash -
RUN apt install -y nodejs
RUN npm install -g tailwindcss css-minify
