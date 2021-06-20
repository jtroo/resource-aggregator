#!/bin/bash
#
# Example script that updates the uptime info in the database for a Linux
# host running on `$HOST_ADDR`.
#
# Dependencies:
# - node
# - curl
# - ssh
# - uptime command available on the target system
# - SSH server running on the target system with the `$HOST_KEY` authorized for
#   `$HOST_USER`

set -eu

HOST_USER=j
HOST_ADDR=192.168.1.100
HOST_KEY=~/.ssh/key_192.168.1.100

# Get current resource info and uptime info from the target
current_info=$(curl http://localhost:8000/resource 2>/dev/null)
uptime=$(ssh -i $HOST_KEY $HOST_USER@$HOST_ADDR uptime)

# This is kinda silly, but whatever, I'm lazy and this was the simplest way I
# could think of to process JSON in bash.
#
# Create a node-runnable script that exits with status 1 if no resource with
# name HOST_ADDRx exists in the info retrieved from the "GET" /resource API.
# The exit code of 1 will result in the bash script ending due to `set -e` at
# the beginning.
#
# Otherwise the Javascript code updates `other_fields.uptime` for the resource
# with name `$HOST_ADDR` then outputs a JSON that can be used in the "POST"
# /resource API. This exits with status 0 to ensure the bash script continues.
JSON_PROCESSING_SCRIPT="
let all_data = $current_info;
for (let resource of all_data) {
	if (resource.name !== '$HOST_ADDR') {
		continue;
	}

	resource.other_fields.uptime = '$uptime';
	console.log(JSON.stringify(resource));
	process.exit(0);
}
console.error('No resource with name $HOST_ADDR was found');
process.exit(1);
"
updated_info=$(node <<< "$JSON_PROCESSING_SCRIPT")

# Update the resource
2>/dev/null curl http://localhost:8000/resource \
	-X POST \
	-H 'Content-Type: application/json' \
	--data "$updated_info"
