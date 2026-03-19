#!/bin/bash
# AuraOS Ollama Build Pipeline
# Sends coding tasks to local Ollama and saves results

OLLAMA_URL="${OLLAMA_URL:-http://localhost:11434}"
MODEL="${MODEL:-qwen2.5-coder:32b}"
OUTPUT_DIR="/Users/venkat/taylorProjects/AuraOS/kernel/src"

generate() {
    local task_name="$1"
    local prompt="$2"
    local output_file="$3"
    
    echo "🔨 Task: $task_name → $output_file"
    echo "   Model: $MODEL"
    
    curl -s "$OLLAMA_URL/api/generate" -d "{
        \"model\": \"$MODEL\",
        \"prompt\": \"$prompt\",
        \"stream\": false,
        \"options\": {\"num_predict\": 6000, \"temperature\": 0.2}
    }" | python3 -c "
import json,sys
try:
    resp = json.load(sys.stdin)
    code = resp.get('response','')
    # Extract code between \`\`\`rust and \`\`\`
    import re
    matches = re.findall(r'\`\`\`rust\n(.*?)\`\`\`', code, re.DOTALL)
    if matches:
        print(matches[0])
    else:
        print(code)
except Exception as e:
    print(f'// Error: {e}', file=sys.stderr)
" > "$output_file"
    
    lines=$(wc -l < "$output_file")
    echo "   ✅ Generated $lines lines → $output_file"
}

echo "=== AuraOS Ollama Build Pipeline ==="
echo "Model: $MODEL"
echo "URL: $OLLAMA_URL"
echo ""

"$@"
