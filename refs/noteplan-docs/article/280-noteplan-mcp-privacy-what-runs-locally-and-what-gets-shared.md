# NotePlan MCP Privacy: What Runs Locally and What Gets Shared

If you haven't set up MCP yet, start here: [How to Install the NotePlan MCP Server](277-how-to-install-the-noteplan-mcp-server.md)

## Summary

NotePlan's MCP server runs on your machine and only exposes local tools to your AI app (Claude Desktop, Claude Code, Codex, etc.). The MCP server itself does not upload your notes anywhere.

## What "runs locally" means

When you connect an AI app to NotePlan via MCP, the AI app talks to a local MCP server process on your Mac. That server can search/read notes and return results back to the AI app, but it does not call any external services by default.

## What data gets sent to external servers (and when)

Your notes are not automatically uploaded to OpenAI/Anthropic/etc.

What can happen instead is:

- The AI app (Claude Desktop / Claude Code / Codex / etc.) asks the local MCP tools to search your notes.
- The search happens locally and the matching results are returned to the AI app.
- If the AI app decides it needs note content to answer your prompt, it may request the content of specific notes and then include that content in the prompt it sends to its model provider (Anthropic/OpenAI/etc.).

So the only content that leaves your machine is whatever the AI app chooses to send as part of the conversation context. This is expected behavior for basically any AI assistant that can read local files.

## Does the MCP server expose my notes publicly?

No. NotePlan's MCP server is designed for local use. It is not meant to be run as a public internet-facing service.

## Sync is separate from MCP

NotePlan sync (CloudKit) is separate from MCP. If you have sync enabled, note changes are synced via Apple's CloudKit. MCP does not change how sync works and does not upload your notes to third parties by itself.

## Optional: embeddings / indexing with an API key

There is an optional feature to index notes using embeddings (for semantic search). This requires configuring an AI API key (for example OpenAI) and explicitly running the indexing flow.

- Default setup (no API key configured): no embeddings are created, and nothing is sent to an embedding provider by NotePlan MCP.
- If you add an API key and run indexing: note content (or chunks of it) will be sent to the embedding provider to build the index. This is opt-in and not enabled by default.

## Practical takeaway

- MCP server: local.
- Searches: local.
- External sharing: only happens when the AI app sends selected note content to its model provider, or if you explicitly enable embeddings/indexing with an API key.

*Last updated: 2026-03-05*
