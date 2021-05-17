#!/bin/bash
cargo build --release
sudo setcap cap_net_admin=eip target/release/rtcp
target/release/rtcp &
pid=$!
sudo ip addr add 192.168.0.1/24 dev rtcp_tun0
sudo ip link set up dev rtcp_tun0
trap "kill $pid" INT TERM
wait $pid
