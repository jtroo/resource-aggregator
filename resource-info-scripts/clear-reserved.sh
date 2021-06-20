#!/bin/bash
#
# Example script that clears the `reserved_*` for all resources if the time
# `reserved_until` field is in the past
#
# Dependencies:
# - node
# - curl


set -eu

# Get current resource info and uptime info from the target
current_info=$(curl http://localhost:8000/resource 2>/dev/null)

JSON_PROCESSING_SCRIPT="
let all_data = $current_info;
let now = Date.now() / 1000;
for (let resource of all_data) {
	// note - reserved_until of 0 has special meaning, so skip it
	if (resource.reserved_until === 0 || resource.reserved_until > now) {
		continue;
	}
	resource.reserved_until = 0;
	resource.reserved_by = '';
	console.log(JSON.stringify(resource));
}
"

all_new_info=$(node <<< "$JSON_PROCESSING_SCRIPT")
if [ -z "$all_new_info" ]; then
	exit 0
fi

for new_info in "$all_new_info"; do
	2>/dev/null curl http://localhost:8000/resource \
		-X POST \
		-H 'Content-Type: application/json' \
		--data "$new_info"
done
