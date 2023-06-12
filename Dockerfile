# Use a base image with Node.js installed
FROM node:lts

# Set the working directory
WORKDIR /yogurt

# Copy the project files to the working directory
COPY . .

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set environment variables
ENV PATH="/root/.cargo/bin:${PATH}"
ENV USER=root

# # Install wasm-pack
RUN cargo install wasm-pack

# # Compile rust to wasm
RUN wasm-pack build . --release --target web --out-dir server/pkg

# Install Nextjs dependancies
WORKDIR /yogurt/server
RUN yarn install

# Build the Nextjs application
RUN yarn build

# Start the Nextjs server
CMD ["yarn","start"]




