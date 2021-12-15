# Reset Spec

Reset chain specs only when we start a chain from block #0 again.

This script uses docker images.

example usage:

```
python3 .maintain/reset-spec/ -i kiltprotocol/kilt-node:develop --alfheim --alfheim-stg --alfheim-dev
python3 .maintain/reset-spec/ -i parity/polkadot:v0.9.10 --alfheim-relay --alfheim-relay-stg
```
