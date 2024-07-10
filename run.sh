#!/bin/bash

set -e

echo "Stopping and removing previous containers (if any)..."
docker stop waku-node1 || true
docker stop waku-node2 || true
docker rm waku-node1 || true
docker rm waku-node2 || true

echo "Creating Docker network..."
docker network create --driver bridge --subnet 172.18.0.0/16 --gateway 172.18.0.1 waku || true

echo "Starting Waku Node 1..."
docker run -d --name waku-node1 \
  --network waku \
  --ip 172.18.111.226 \
  -p 21161:21161 \
  -p 21162:21162 \
  -p 21163:21163 \
  -p 21164:21164 \
  -p 21165:21165 \
  wakuorg/nwaku:v0.24.0 \
  --listen-address=0.0.0.0 \
  --rest=true \
  --rest-admin=true \
  --websocket-support=true \
  --log-level=TRACE \
  --rest-relay-cache-capacity=100 \
  --websocket-port=21163 \
  --rest-port=21161 \
  --tcp-port=21162 \
  --discv5-udp-port=21164 \
  --rest-address=0.0.0.0 \
  --nat=extip:172.18.111.226 \
  --peer-exchange=true \
  --discv5-discovery=true \
  --relay=true

echo "Verifying Waku Node 1 Info..."
sleep 5
response=$(curl -s --location 'http://localhost:21161/debug/v1/info')
echo "$response"
enrUri=$(echo $response | jq -r '.enrUri')

echo "Starting Waku Node 2..."
docker run -d --name waku-node2 \
  --network waku \
  --ip 172.18.111.227 \
  -p 21261:21161 \
  -p 21262:21162 \
  -p 21263:21163 \
  -p 21264:21164 \
  -p 21265:21165 \
  wakuorg/nwaku:v0.24.0 \
  --listen-address=0.0.0.0 \
  --rest=true \
  --rest-admin=true \
  --websocket-support=true \
  --log-level=TRACE \
  --rest-relay-cache-capacity=100 \
  --websocket-port=21263 \
  --rest-port=21161 \
  --tcp-port=21262 \
  --discv5-udp-port=21264 \
  --rest-address=0.0.0.0 \
  --nat=extip:172.18.111.227 \
  --peer-exchange=true \
  --discv5-discovery=true \
  --relay=true \
  --discv5-bootstrap-node=$enrUri

echo "Attaching Nodes to Network..."
docker network connect --ip 172.18.111.226 waku waku-node1 || true
docker network connect --ip 172.18.111.227 waku waku-node2 || true

echo "Verifying Auto-Connection..."
connected=false
for i in {1..20}; do
  sleep 60
  response=$(curl -s --location 'http://localhost:21161/admin/v1/peers')
  echo "$response"
  if echo $response | jq -e '.[] | select(.protocols[] | .connected == true)' > /dev/null; then
    connected=true
    break
  fi
done

if [ "$connected" == "true" ]; then
  echo "Nodes are connected successfully!"
else
  echo "Nodes failed to connect within the given time."
  exit 1
fi

echo "Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
export PATH="$HOME/.cargo/bin:$PATH"

echo "Building and Testing Rust Code..."
cd "$(dirname "$0")"
cargo build --release
cargo test --test integration_tests -- --test-threads=1

echo "Done!"
