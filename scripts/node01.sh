#!/bin/bash

../target/release/proofid-node --base-path /var/substrate/node01 --chain ./customSpecRaw.json --rpc-cors all --port 30333 --ws-port 9945 --rpc-port 9933 --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' --validator --name Node01 --rpc-methods Unsafe --unsafe-rpc-external --unsafe-ws-external
