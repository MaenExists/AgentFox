#!/bin/bash
# AgentFox Demo Script
# Run this to see AgentFox in action!

echo "🦊 AgentFox Demo - High-velocity browser for AI agents"
echo "===================================================="

# Start fresh
echo "🔄 Ensuring clean state..."
afox quit 2>/dev/null || true

echo ""
echo "🔍 1. Smart Search & Navigation"
echo "   Running: afox search \"open source ai agents framework\""
afox search "open source ai agents framework"
sleep 2

echo ""
echo "👀 2. Inspect Page (Agent-Friendly View)"
echo "   Running: afox view"
afox view
sleep 2

echo ""
echo "🖱️  3. Interact with Elements (Direct-ID)"
echo "   Looking for GitHub link in results..."
# Let's try to find and click a result
echo "   Running: afox click e15 2>/dev/null || echo 'Trying different approach...'"
afox click e15 2>/dev/null || echo "   (Element ID may vary - this is expected in demo)"
sleep 2

echo ""
echo "📝 4. Form Interaction"
echo "   Navigating to a test form..."
afox open https://httpbin.org/forms/post
sleep 2
afox view
sleep 2
echo "   Filling email field..."
afox fill e5 "demo@agentfox.dev"
sleep 1
echo "   Checking selection..."
afox text e5
sleep 1

echo ""
echo "⚡ 5. JavaScript Execution"
echo "   Running: afox eval \"document.title\""
afox eval "document.title"
sleep 1

echo ""
echo "📊 6. Performance Test"
echo "   Timing repeated operations..."
TIMEFORMAT='%3R'
time (
  afox search "rust programming" >/dev/null 2>&1
  afox view >/dev/null 2>&1
  afox quit >/dev/null 2>&1
)

echo ""
echo "✅ Demo Complete!"
echo "💡 Key Features Demonstrated:"
echo "   • Fast search & navigation (~1-2s)"
echo "   • Clean markdown snapshots with stable IDs"
echo "   • Direct-ID element interaction (no brittle selectors)"
echo "   • Form filling & JS execution"
echo "   • Persistent daemon with clean shutdown"
echo ""
echo "🚀 Ready for your AI agent workflows!"
echo "   Install: curl -sSL https://raw.githubusercontent.com/MaenExists/AgentFox/main/install.sh | bash"