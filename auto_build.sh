#!/bin/bash
# AuraOS Auto-Build: merges completed work + dispatches new tasks continuously
cd /Users/venkat/taylorProjects/AuraOS

MODULES=(
    "audit_log" "seccomp_filter" "crypto_engine" "tls_handshake"
    "dhcp_client" "ntp_sync" "route_table" "arp_cache"
    "packet_filter" "nat_gateway" "proxy_server" "websocket_mgr"
    "grpc_runtime" "agent_debugger" "agent_profiler" "agent_snapshot"
    "agent_checkpoint" "intent_parser" "nlp_tokenizer" "embedding_engine"
    "rag_pipeline" "tool_executor" "model_loader" "tensor_ops"
    "quantizer" "batch_scheduler" "priority_queue" "deadlock_detector"
    "memory_pool" "slab_allocator" "buddy_allocator" "bitmap_alloc"
    "vga_text" "framebuffer_mgr" "cursor_manager" "clipboard_mgr"
    "drag_drop" "tooltip_system" "menu_system" "dialog_box"
    "progress_bar" "scrollbar" "tab_control" "tree_view"
    "file_dialog" "color_picker" "date_picker" "notification_center"
    "system_tray" "taskbar"
)

NODES=(
    "sophie@192.168.5.103:sophie"
    "marcus@192.168.5.102:marcus"
    "james@192.168.5.108:james"
    "priya@192.168.5.106:priya"
    "ace@192.168.5.104:ace"
)

IDX=0
TOTAL=${#MODULES[@]}
declare -A NODE_PID
declare -A NODE_MODULE

dispatch() {
    local node_full=$1
    local node=$(echo $node_full | cut -d: -f1)
    local user=$(echo $node_full | cut -d: -f2)
    
    if [ $IDX -ge $TOTAL ]; then return 1; fi
    local module=${MODULES[$IDX]}
    IDX=$((IDX + 1))
    
    python3 -c "
import json
req = {'model': 'qwen2.5-coder:14b', 'prompt': 'Write ONLY Rust code for a kernel module, no markdown, no code fences. extern crate alloc; use alloc::string::String; use alloc::vec::Vec; Create a complete working module called ${module} for an AI-native operating system kernel. Include at least one public struct with an impl block, a new() constructor, and at least 5 useful public methods with real logic. Use only no_std compatible types. No format!, no println!, no std::, no HashMap, no no_mangle.', 'stream': False, 'options': {'num_predict': 3000, 'temperature': 0.15}}
json.dump(req, open('/tmp/pipe_${module}.json', 'w'))
"
    scp /tmp/pipe_${module}.json ${node}:/tmp/prompt.json 2>/dev/null
    
    if [ "$user" = "ace" ]; then
        ssh ${node} "export PATH=\"/usr/local/bin:\$HOME/.cargo/bin:\$PATH\"; cd ~/AuraOS && git checkout main && git fetch origin && git reset --hard origin/main && bash ~/aura_build.sh ${module} /tmp/prompt.json" &
    else
        ssh ${node} "cd ~/AuraOS && git checkout main && git fetch origin && git reset --hard origin/main && bash ~/aura_build.sh ${module} /tmp/prompt.json" &
    fi
    NODE_PID[$user]=$!
    NODE_MODULE[$user]=$module
    echo "[$(date +%H:%M:%S)] DISPATCH $user → $module ($IDX/$TOTAL)"
    return 0
}

merge_and_commit() {
    cd /Users/venkat/taylorProjects/AuraOS
    git fetch origin --prune 2>/dev/null
    local merged=0
    
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
                merged=$((merged + 1))
                echo "[$(date +%H:%M:%S)] MERGED $name"
            fi
        fi
    done
    
    if [ $merged -gt 0 ]; then
        cd kernel/src
        python3 -c "
import os
modules = sorted([f.replace('.rs','') for f in os.listdir('.') if f.endswith('.rs') and f != 'main.rs'])
lines = [l for l in open('main.rs').readlines() if not l.startswith('mod ')]
idx = 0
for i,l in enumerate(lines):
    if 'fn kernel_main' in l: idx = i; break
open('main.rs','w').write(''.join(lines[:idx]) + '\n'.join(['mod '+m+';' for m in modules]) + '\n\n' + ''.join(lines[idx:]))
print(f'{len(modules)} modules')
"
        cd /Users/venkat/taylorProjects/AuraOS
        git add -A && git commit -m "feat: auto-build batch ($merged modules)" 2>/dev/null | head -1
        git push origin main 2>/dev/null | tail -1
        wc -l kernel/src/*.rs | tail -1
    fi
}

# Initial dispatch
for node_info in "${NODES[@]}"; do
    dispatch "$node_info" || break
done

# Main loop
while true; do
    sleep 10
    
    any_running=false
    for node_info in "${NODES[@]}"; do
        user=$(echo $node_info | cut -d: -f2)
        pid=${NODE_PID[$user]}
        
        if [ -n "$pid" ] && ! kill -0 $pid 2>/dev/null; then
            echo "[$(date +%H:%M:%S)] DONE $user finished ${NODE_MODULE[$user]}"
            NODE_PID[$user]=""
            
            # Merge immediately
            merge_and_commit
            
            # Dispatch next
            dispatch "$node_info" || true
        fi
        
        if [ -n "${NODE_PID[$user]}" ] && kill -0 ${NODE_PID[$user]} 2>/dev/null; then
            any_running=true
        fi
    done
    
    if ! $any_running && [ $IDX -ge $TOTAL ]; then
        merge_and_commit
        echo "[$(date +%H:%M:%S)] === ALL COMPLETE ==="
        break
    fi
done

echo "=== FINAL STATUS ==="
cd /Users/venkat/taylorProjects/AuraOS
wc -l kernel/src/*.rs | tail -1
ls kernel/src/*.rs | wc -l
git log --oneline | wc -l
