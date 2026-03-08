#!/usr/bin/env bash
# This works on Linux and MacOS to build production Artcraft

set -euxo pipefail

# OpenClaw non-login shells may not include Rust toolchain bins in PATH.
export PATH="$HOME/.cargo/bin:$PATH"

echo "Building production Artcraft..."
echo ""

root_dir=$(pwd)
frontend_path="${root_dir}/frontend"
rust_crate_path="${root_dir}/crates/desktop/artcraft"

# The tauri dev server integration is super annoying: It eats ctrl-c interrupts,
# it decoheres the and corrupts terminal output, and it's slow. This configuration
# allows us to start without it. Simply launch `nx` and the dev server as a separate
# process and leave tauri out of the loop
config_path="${rust_crate_path}/tauri.conf.json"

pushd "${frontend_path}" || exit

npm install --verbose
npx nx sync

popd || exit

export TAURI_FRONTEND_PATH="${frontend_path}"
export TAURI_APP_PATH="${rust_crate_path}"

# NB: The "frontend dev" script sets "production" too, so this must only control the
# hostnames we use, not minification, etc.
export VITE_ENVIRONMENT_TYPE="production"

# Keep memory usage lower on constrained hosts to prevent rustc SIGKILL/OOM.
export CARGO_BUILD_JOBS=1

# This appears to trigger "nx build" instead of "nx dev".
#
# NOTE (OpenClaw): RPM bundling appears to hang on some hosts. By default on Linux we
# bundle **deb only** to ensure local builds complete. Override with:
#   ARTCRAFT_TAURI_BUNDLES="deb,rpm" ./script/artcraft/unix_build.sh
# or to skip bundling entirely:
#   ARTCRAFT_TAURI_NO_BUNDLE=1 ./script/artcraft/unix_build.sh
bundles_args=()
if [[ "${ARTCRAFT_TAURI_NO_BUNDLE:-}" == "1" ]]; then
  bundles_args+=(--no-bundle)
elif [[ "$(uname)" == "Linux" ]]; then
  bundles="${ARTCRAFT_TAURI_BUNDLES:-deb}"
  bundles_args+=(--bundles "${bundles}")
fi

cargo tauri build --config "${config_path}" "${bundles_args[@]}"

echo "Done!"

date "+Finished on %A, %B %e - %H:%M:%S (local timezone)"

