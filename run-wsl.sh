#! /bin/bash

# force vulkan backend for wgpu on WSL2
export WGPU_BACKEND=vulkan
export MESA_VK_IGNORE_CONFORMANCE_WARNING=1

# disable wayland in order to force x11
unset WAYLAND_DISPLAY

cargo run -r -- "$@"
