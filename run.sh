#!/bin/bash
cargo build 
sudo setcap cap_net_admin=eip target/debug/rtcp
target/debug/rtcp &
pid=$!
sudo ip addr add 192.168.0.1/24 dev rtcp_tun0
sudo ip link set up dev rtcp_tun0
trap "kill $pid" INT TERM
wait $pid
