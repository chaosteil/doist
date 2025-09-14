# Todoist CLI (`doist`) Instructions

Task management CLI for Todoist.

## Authentication
```bash
doist auth <key>  # Get API key from app.todoist.com/app/settings/integrations/developer
```

## Basic Commands
```bash
doist                              # Today/overdue tasks (default filter)
doist list -f all                  # ALL tasks (important: default only shows today/overdue)
doist add "task" -P Work -d today # Add task with project and due date
doist close <id>                   # Complete task
```

## Important Notes
- **Default behavior**: Shows only today/overdue tasks
- **View ALL tasks**: Use `doist list -f all` to see everything
- **Details**: See `commands/todoist.md` for projects/labels reference