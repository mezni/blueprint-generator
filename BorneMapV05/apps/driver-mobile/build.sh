#!/bin/bash

# Driver Mobile App Build Script

set -e  # Exit on error

echo "=== Bornemap Driver Mobile App Build Script ==="
echo ""

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if node_modules exists
if [ ! -d "node_modules" ]; then
    print_info "Node modules not found. Installing dependencies..."
    npm install
    print_info "Dependencies installed successfully"
else
    print_info "Node modules found"
fi

# Check if .env file exists
if [ ! -f ".env" ]; then
    print_warning ".env file not found"
    print_info "Creating .env from .env.example..."
    if [ -f ".env.example" ]; then
        cp .env.example .env
        print_info ".env file created. Please edit it with your configuration."
    else
        print_error ".env.example not found"
        exit 1
    fi
else
    print_info ".env file found"
fi

# Build command based on platform
case "$1" in
    ios)
        print_info "Building for iOS..."
        npm run ios
        ;;
    android)
        print_info "Building for Android..."
        npm run android
        ;;
    web)
        print_info "Building for Web..."
        npm run web
        ;;
    clean)
        print_info "Cleaning project..."
        rm -rf node_modules
        rm -rf .expo
        rm -rf android/app/build
        rm -rf ios/Pods
        print_info "Cleanup complete"
        exit 0
        ;;
    dev)
        print_info "Starting development server..."
        npm run dev
        ;;
    *)
        print_info "Available commands:"
        echo "  ./build.sh ios        - Build for iOS"
        echo "  ./build.sh android    - Build for Android"
        echo "  ./build.sh web        - Build for Web"
        echo "  ./build.sh dev        - Start development server"
        echo "  ./build.sh clean      - Clean project"
        exit 0
        ;;
esac
