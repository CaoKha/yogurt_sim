# yogurt_sim
A yogurt simulation

![ohaio](https://media.tenor.com/G78em4XTdjwAAAAd/yogurt.gif)

## Compilation

```bash
chmod +x compile.sh
./compile.sh
```

This command will generate a `pkg/` folder containing wasm and js files for the nextjs server.
When a code is changed in lib.rs, remember run 
```bash
./compile.sh

```
again and remove the `.next/` and `node_modules/` folder in order to remove any existing caches:
```bash
cd server/
rm -r .next node_modules 
```
Then, install all the packages and run the server again:
```bash
yarn install
yarn run dev
```

