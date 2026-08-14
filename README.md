# Pulse

## How to Use

### Installation

\`\`\`base
cargo install --path pulse
\`\`\`

(Workspace member - installs the `pulse` binary from the `pulse` crate.)

### Logging your work

Add a manual entry any time:

\`\`\`bash
pulse log "fixed decoder bug"
pulse log "reviewed PR #44" --kind work
\`\`\`

### Viewing summaries

Log consists of nudges completed + manual entries.

\`\`\`bash
pulse today 
pulse week
\`\`\`

### Configuring nudges

\`\`\`bash
pulse config    # show current nudge config
\`\`\`

Nudges (water, stand up, stretch) are defined in `~/.config/pulse/config.toml`:


\`\`\`toml
[[nudge]]
name = "water"
interval = "1hr"

[[nudge]]
name = "stretch"
interval = "30m"
\`\`\`
