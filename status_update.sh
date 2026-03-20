#!/bin/bash
cd /Users/venkat/taylorProjects/AuraOS

LINES=$(wc -l kernel/src/*.rs | tail -1 | awk '{print $1}')
MODULES=$(ls kernel/src/*.rs | wc -l | tr -d ' ')
COMMITS=$(git log --oneline | wc -l | tr -d ' ')
LAST_HOUR=$(git log --oneline --since="15 minutes ago" | wc -l | tr -d ' ')
QUEUE=$(wc -l < /tmp/aura_module_queue.txt | tr -d ' ')

# Get recent new modules
RECENT=$(git log --since="15 minutes ago" --name-only --diff-filter=A -- 'kernel/src/*.rs' | grep '.rs' | sed 's/kernel\/src\///' | sed 's/.rs//' | tr '\n' ', ' | sed 's/,$//')

# Node status
NODES=""
for node in sophie@192.168.5.103 marcus@192.168.5.102 james@192.168.5.108 priya@192.168.5.106 ace@192.168.5.104; do
    user=$(echo $node | cut -d@ -f1)
    branch=$(ssh $node "cd ~/AuraOS && git branch --show-current 2>/dev/null" 2>/dev/null | sed 's/feat\///')
    running=$(ssh $node "ps aux | grep aura_build | grep -v grep" 2>/dev/null)
    if [ -n "$running" ]; then
        NODES="${NODES}${user}→${branch} "
    else
        NODES="${NODES}${user}(idle) "
    fi
done

echo "${LINES}|${MODULES}|${COMMITS}|${LAST_HOUR}|${QUEUE}|${RECENT}|${NODES}"
