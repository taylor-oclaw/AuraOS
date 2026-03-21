#!/bin/bash
cd /Users/venkat/taylorProjects/AuraOS
git fetch origin --prune 2>/dev/null

MODULES_QUEUE="/tmp/aura_module_queue.txt"
LOCKFILE="/tmp/aura_merge.lock"

# Don't run if already running
if [ -f "$LOCKFILE" ]; then exit 0; fi
touch "$LOCKFILE"
trap "rm -f $LOCKFILE" EXIT

# Check for unmerged branches
MERGED=0
for branch in $(git branch -r | grep "feat/" | sed 's/origin\///'); do
    name=$(echo $branch | sed 's/feat\///')
    if [ ! -f "kernel/src/${name}.rs" ]; then
        git show origin/$branch:kernel/src/${name}.rs > kernel/src/${name}.rs 2>/dev/null
        if [ -f "kernel/src/${name}.rs" ]; then
            python3 -c "
import re
c = open('kernel/src/${name}.rs').read()
if 'format!' in c: c = re.sub(r'format!\([^)]*\)', 'String::from(\"info\")', c)
lines = [l for l in c.split('\n') if 'println!' not in l and '#[no_mangle]' not in l]
fixed = []
for line in lines:
    s = line.rstrip()
    while s.count(')') > s.count('('): s = s[:s.rfind(')')] + s[s.rfind(')')+1:]
    fixed.append(s)
c = '\n'.join(fixed)
if 'vec!' in c and 'use alloc::vec;' not in c and 'use alloc::vec::Vec;' in c:
    c = c.replace('use alloc::vec::Vec;', 'use alloc::vec::Vec;\nuse alloc::vec;')
open('kernel/src/${name}.rs', 'w').write(c)
" 2>/dev/null
            MERGED=$((MERGED + 1))
        fi
    fi
done

if [ $MERGED -gt 0 ]; then
    cd kernel/src
    python3 -c "
import os
modules = sorted([f.replace('.rs','') for f in os.listdir('.') if f.endswith('.rs') and f != 'main.rs'])
lines = [l for l in open('main.rs').readlines() if not l.startswith('mod ')]
idx = 0
for i,l in enumerate(lines):
    if 'fn kernel_main' in l: idx = i; break
open('main.rs','w').write(''.join(lines[:idx]) + '\n'.join(['mod '+m+';' for m in modules]) + '\n\n' + ''.join(lines[idx:]))
"
    cd /Users/venkat/taylorProjects/AuraOS
    git add -A && git commit -m "auto-merge: $MERGED new modules" 2>/dev/null
    git push origin main 2>/dev/null
fi

# Check idle nodes and dispatch from queue
if [ -f "$MODULES_QUEUE" ] && [ -s "$MODULES_QUEUE" ]; then
    for node_info in "localhost:taylor" "sophie@192.168.5.103:sophie" "marcus@192.168.5.102:marcus" "james@192.168.5.108:james" "priya@192.168.5.106:priya" "ace@192.168.5.104:ace"; do
        node=$(echo $node_info | cut -d: -f1)
        user=$(echo $node_info | cut -d: -f2)
        if [ "$node" = "localhost" ]; then
            running=$(ps aux | grep aura_build | grep -v grep | grep -v ssh 2>/dev/null)
        else
            running=$(ssh $node "ps aux | grep aura_build | grep -v grep" 2>/dev/null)
        fi
        if [ -z "$running" ]; then
            module=$(head -1 "$MODULES_QUEUE")
            if [ -n "$module" ]; then
                sed -i '' '1d' "$MODULES_QUEUE" 2>/dev/null || sed -i '1d' "$MODULES_QUEUE" 2>/dev/null
                python3 -c "
import json
req = {'model': 'qwen2.5-coder:14b', 'prompt': 'Write ONLY Rust code for a kernel module, no markdown, no code fences. extern crate alloc; use alloc::string::String; use alloc::vec::Vec; Create a complete working module called ${module} for an AI-native operating system kernel. Include at least one public struct with an impl block, a new() constructor, and at least 5 useful public methods with real logic. Use only no_std compatible types. No format!, no println!, no std::, no HashMap, no no_mangle, no asm!, no rand.', 'stream': False, 'options': {'num_predict': 3000, 'temperature': 0.15}}
json.dump(req, open('/tmp/pipe_${module}.json', 'w'))
"
                if [ "$node" = "localhost" ]; then
                    cp /tmp/pipe_${module}.json /tmp/prompt.json
                    (cd ~/AuraOS && git checkout main && git fetch origin && git reset --hard origin/main && bash ~/aura_build.sh ${module} /tmp/prompt.json) </dev/null >/dev/null 2>&1 &
                elif [ "$user" = "ace" ]; then
                    scp /tmp/pipe_${module}.json ${node}:/tmp/prompt.json 2>/dev/null
                    ssh $node "export PATH=\"/usr/local/bin:\$HOME/.cargo/bin:\$PATH\"; cd ~/AuraOS && git checkout main && git fetch origin && git reset --hard origin/main && bash ~/aura_build.sh ${module} /tmp/prompt.json" </dev/null >/dev/null 2>&1 &
                else
                    scp /tmp/pipe_${module}.json ${node}:/tmp/prompt.json 2>/dev/null
                    ssh $node "cd ~/AuraOS && git checkout main && git fetch origin && git reset --hard origin/main && bash ~/aura_build.sh ${module} /tmp/prompt.json" </dev/null >/dev/null 2>&1 &
                fi
                echo "$(date +%H:%M) DISPATCH $user → $module" >> /tmp/aura_dispatch.log
            fi
        fi
    done
fi
