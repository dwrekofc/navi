# How to Install the NotePlan MCP Server

The NotePlan MCP server connects NotePlan to AI clients, so you can manage notes, tasks, reminders, and calendar items by chat.

## What You Need

- A Mac with NotePlan installed
- Node.js 18 or newer
- One of these clients:
- Claude Desktop: [https://claude.ai/download](https://claude.ai/download)
- Claude Code: [https://docs.anthropic.com/en/docs/claude-code/quickstart](https://docs.anthropic.com/en/docs/claude-code/quickstart)
- OpenAI Codex (macOS): [https://openai.com/index/introducing-the-codex-app/](https://openai.com/index/introducing-the-codex-app/)

Tip: For the latest MCP features, use the [NotePlan TestFlight build](https://testflight.apple.com/join/fm9q4OjE).

## Step 1 — Install Node.js

If you don't have Node.js yet, install it from [https://nodejs.org](https://nodejs.org).

## Step 2 — Connect NotePlan MCP

### Claude Desktop

- Open Claude Desktop.
- Go to Settings > Developer > Edit Config.
- Replace the config with:

```
{
  "mcpServers": {
    "noteplan": {
      "command": "npx",
      "args": ["-y", "@noteplanco/noteplan-mcp"]
    }
  }
}
```

- Save and restart Claude Desktop.

### Claude Code

Run:

```
claude mcp add noteplan -- npx -y @noteplanco/noteplan-mcp
claude mcp list
```

### OpenAI Codex (CLI)

Codex runs with your ChatGPT account/subscription.

Run:

```
codex mcp add noteplan -- npx -y @noteplanco/noteplan-mcp
codex mcp list
```

### OpenAI Codex (UI)

Go to Settings > MCP servers > Add new server, then fill:

- Name: `noteplan`
- Command to launch: `npx` (or `/usr/local/bin/npx`)
- Arguments: `-y, @noteplanco/noteplan-mcp`
- Working directory: keep default (for example `~/code`)

## Step 3 — Try It

- "What's on my schedule today?"
- "Show my open tasks tagged #urgent."
- "Add a task to my Daily Note: call the dentist at 3pm."

The `npx` command automatically downloads and runs the NotePlan MCP server.

## Troubleshooting

### How to view MCP logs (Claude Desktop)

Logs are the fastest way to diagnose MCP issues. To find them:

1. Open Claude Desktop **Settings** (⌘ + , on Mac).
2. Click **Developer**, then click the **NotePlan** MCP server entry.
3. Click **View Logs** — this opens the log file in Console.app.
4. In Console.app, click **Reveal in Finder** (at the top) to see the log file on disk.

The log file is typically located at:
`~/Library/Logs/Claude/mcp-server-noteplan.log`

### "No such file or directory" on startup

If Claude Desktop shows `env: node: No such file or directory` or `Failed to spawn process`, it usually means Node.js was installed with a version manager (like `n`, `nvm`, `fnm`, or `volta`) and Claude Desktop can't find it.

To fix this, open Terminal and run:

```
which npx
```

This will show the full path, for example `/Users/you/.n/bin/npx`. Take the folder part (`/Users/you/.n/bin`) and update your Claude Desktop config:

```
{
  "mcpServers": {
    "noteplan": {
      "command": "npx",
      "args": ["-y", "@noteplanco/noteplan-mcp"],
      "env": {
        "PATH": "/Users/you/.n/bin:/usr/local/bin:/opt/homebrew/bin:/usr/bin:/bin:/usr/sbin:/sbin"
      }
    }
  }
}
```

Replace `/Users/you/.n/bin` with the folder from your `which npx` output. Save and restart Claude Desktop.

### Claude isn't using the NotePlan tools

If Claude responds to your question without using the NotePlan tools (you don't see a tool use indicator in the response), try these steps in order:

1. **Delete the MCP log file.** Find it using the steps under "How to view MCP logs" above, then delete the file.
2. **Fully restart Claude Desktop.** Quit with ⌘Q (not just close the window), then relaunch.
3. **Start a new conversation.** Open a fresh **Chat** conversation (not Cowork).
4. **Tell the AI to refresh.** In your new conversation, try sending: *"The NotePlan MCP tools have been updated. Please refresh your tool list and show me my calendar events for today."* — this nudges the AI to re-check the available tools instead of relying on cached information.

Claude Desktop can sometimes cache stale MCP tool metadata. Restarting and starting a fresh conversation clears this.

### Calendar or Reminders access not working

The MCP server accesses your calendars and reminders through NotePlan via AppleScript. For this to work:

- **NotePlan must be running.** If NotePlan isn't open, calendar and reminder commands will fail.
- **NotePlan must be up to date.** Calendar and reminder support requires NotePlan 3.20.2 or later. Install the latest version from the App Store or use the [TestFlight build](https://testflight.apple.com/join/fm9q4OjE).

### Sending logs for support

If something isn't working, we may ask you for your MCP logs. To get a clean log:

1. Quit Claude Desktop (⌘Q).
2. Delete the existing log file (see "How to view MCP logs" above for the location).
3. Relaunch Claude Desktop.
4. Open a new Chat conversation and reproduce the issue.
5. Open the new log file and send us the full contents.

A fresh log makes it much easier to diagnose the problem.

*Last updated: 2026-02-27*
