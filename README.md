# Solana NFT (Demo)

## Deploying

1. Installing dependencies:
    ```commandline
    npm install
    ```

2. Building .so file
    ```commandline
    npm run build:program-rust
    ```

3. Deploying contract
    ```commandline
    solana program deploy dist/program/nft.so
    ```