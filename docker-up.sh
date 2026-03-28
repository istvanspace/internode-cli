#!/bin/bash
set -e

cd "$(dirname "$0")"

IMAGE_NAME="internode-cli"

echo "Building internode CLI Docker image..."
docker build -t "$IMAGE_NAME" .

echo ""
echo "============================================"
echo "  internode CLI is ready"
echo "============================================"
echo ""
echo "  Quick start:"
echo "    1. Get an API key from Settings > CLI API Key"
echo "       at https://app.internode.work"
echo ""
echo "    2. Inside the container, run:"
echo "       internode configure <your-api-key>"
echo ""
echo "    3. Verify:"
echo "       internode auth status"
echo ""
echo "  Dropping you into the container now..."
echo ""

docker run -it --rm \
    --name internode-cli \
    --network host \
    "$IMAGE_NAME"
