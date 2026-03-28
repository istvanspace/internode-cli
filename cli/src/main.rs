mod client;
mod commands;
mod config;
mod error;
mod output;

use clap::{Parser, Subcommand};

use crate::error::CliError;

#[derive(Parser)]
#[command(
    name = "internode",
    about = "Agent-native CLI for Internode Organizational Intelligence",
    version,
    propagate_version = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Save your CLI API key (get one from Settings > CLI API Key in the portal)
    Configure {
        /// Your CLI API key (starts with ink_)
        api_key: String,
        /// Override the API base URL
        #[arg(long = "api-url")]
        api_url: Option<String>,
    },
    /// Authentication commands
    Auth {
        #[command(subcommand)]
        command: AuthCmd,
    },
    /// Browse OI topics
    Topics {
        #[command(subcommand)]
        command: TopicsCmd,
    },
    /// Browse OI sub-topics (Ideas, Problems, Solutions, etc.)
    Subtopics {
        #[command(subcommand)]
        command: SubtopicsCmd,
    },
    /// Browse and update OI tasks
    Tasks {
        #[command(subcommand)]
        command: TasksCmd,
    },
    /// Browse OI decisions
    Decisions {
        #[command(subcommand)]
        command: DecisionsCmd,
    },
    /// Browse OI intents
    Intents {
        #[command(subcommand)]
        command: IntentsCmd,
    },
    /// Retrieve detailed entity information (knowledge molecules)
    Entity {
        #[command(subcommand)]
        command: EntityCmd,
    },
    /// Browse OI teams
    Teams {
        #[command(subcommand)]
        command: TeamsCmd,
    },
    /// Browse and create OI projects
    Projects {
        #[command(subcommand)]
        command: ProjectsCmd,
    },
    /// Browse OI statuses
    Statuses {
        #[command(subcommand)]
        command: StatusesCmd,
    },
    /// Semantic search over organizational intelligence
    Search {
        /// Search query text
        query: String,
        /// Number of results to return
        #[arg(long = "top-k")]
        top_k: Option<i64>,
        /// Minimum similarity score (0.0 to 1.0)
        #[arg(long = "min-score")]
        min_score: Option<f64>,
    },
    /// Dump structural OI context optimized for LLM consumption
    Context {
        /// Maximum token budget for context output
        #[arg(long = "max-tokens")]
        max_tokens: Option<i64>,
    },
}

#[derive(Subcommand)]
enum AuthCmd {
    /// Verify API key and show account info
    Status,
}

#[derive(Subcommand)]
enum TopicsCmd {
    /// List topics with optional filters
    List {
        #[arg(long)]
        limit: Option<i64>,
        #[arg(long)]
        offset: Option<i64>,
        #[arg(long)]
        search: Option<String>,
        /// Filter by topic category index
        #[arg(long)]
        category: Option<i64>,
    },
}

#[derive(Subcommand)]
enum SubtopicsCmd {
    /// List sub-topics with optional filters
    List {
        /// Sub-topic type (Idea, Problem, Solution, Information, Outcome, etc.)
        #[arg(long = "type")]
        type_filter: Option<String>,
        /// Filter by parent topic ID
        #[arg(long)]
        topic: Option<String>,
        #[arg(long)]
        limit: Option<i64>,
        #[arg(long)]
        offset: Option<i64>,
    },
}

#[derive(Subcommand)]
enum TasksCmd {
    /// List tasks with optional filters
    List {
        #[arg(long)]
        team: Option<String>,
        #[arg(long)]
        project: Option<String>,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        assignee: Option<String>,
        #[arg(long)]
        priority: Option<String>,
        #[arg(long)]
        search: Option<String>,
        /// Filter by related topic ID
        #[arg(long)]
        topic: Option<String>,
        /// Filter by related intent ID
        #[arg(long)]
        intent: Option<String>,
        /// Filter by topic category
        #[arg(long = "topic-category")]
        topic_category: Option<String>,
        #[arg(long)]
        limit: Option<i64>,
        #[arg(long)]
        offset: Option<i64>,
    },
    /// Update an existing task
    Update {
        id: String,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        priority: Option<String>,
        #[arg(long)]
        assignee: Option<String>,
        #[arg(long = "due-date")]
        due_date: Option<String>,
        /// Status ID to assign
        #[arg(long)]
        status: Option<String>,
        /// Team ID to assign (auto-clears incompatible project/status/assignee)
        #[arg(long)]
        team: Option<String>,
        /// Project ID to assign (must belong to task's team)
        #[arg(long)]
        project: Option<String>,
        #[arg(long = "user-notes")]
        user_notes: Option<String>,
        #[arg(long = "blocked-by-reason")]
        blocked_by_reason: Option<String>,
        #[arg(long = "type")]
        task_type: Option<String>,
    },
}

#[derive(Subcommand)]
enum DecisionsCmd {
    /// List decisions with optional filters
    List {
        #[arg(long)]
        search: Option<String>,
        #[arg(long)]
        limit: Option<i64>,
        #[arg(long)]
        offset: Option<i64>,
    },
}

#[derive(Subcommand)]
enum IntentsCmd {
    /// List intents
    List {
        #[arg(long)]
        limit: Option<i64>,
        #[arg(long)]
        offset: Option<i64>,
    },
}

#[derive(Subcommand)]
enum EntityCmd {
    /// Get detailed info for one or more entities (max 20)
    Get {
        /// Entity IDs to retrieve
        #[arg(required = true, num_args = 1..=20)]
        ids: Vec<String>,
    },
}

#[derive(Subcommand)]
enum TeamsCmd {
    /// List teams
    List,
}

#[derive(Subcommand)]
enum ProjectsCmd {
    /// List projects
    List {
        #[arg(long)]
        team: Option<String>,
    },
    /// Create a new project
    Create {
        #[arg(long)]
        name: String,
        /// Team ID (required)
        #[arg(long)]
        team: String,
        #[arg(long)]
        key: Option<String>,
        #[arg(long)]
        description: Option<String>,
    },
}

#[derive(Subcommand)]
enum StatusesCmd {
    /// List statuses
    List {
        #[arg(long)]
        team: Option<String>,
    },
}

async fn run(cli: Cli) -> Result<(), CliError> {
    match cli.command {
        Commands::Configure { api_key, api_url } => {
            commands::auth::configure(&api_key, api_url.as_deref()).await
        }
        Commands::Auth { command } => match command {
            AuthCmd::Status => commands::auth::status().await,
        },
        Commands::Topics { command } => match command {
            TopicsCmd::List { limit, offset, search, category } => {
                commands::topics::list(limit, offset, search.as_deref(), category).await
            }
        },
        Commands::Subtopics { command } => match command {
            SubtopicsCmd::List { type_filter, topic, limit, offset } => {
                commands::subtopics::list(type_filter.as_deref(), topic.as_deref(), limit, offset).await
            }
        },
        Commands::Tasks { command } => match command {
            TasksCmd::List { team, project, status, assignee, priority, search, topic, intent, topic_category, limit, offset } => {
                commands::tasks::list(
                    team.as_deref(), project.as_deref(), status.as_deref(),
                    assignee.as_deref(), priority.as_deref(), search.as_deref(),
                    topic.as_deref(), intent.as_deref(), topic_category.as_deref(),
                    limit, offset,
                ).await
            }
            TasksCmd::Update { id, title, description, priority, assignee, due_date, status, team, project, user_notes, blocked_by_reason, task_type } => {
                commands::tasks::update(
                    &id, title.as_deref(), description.as_deref(), priority.as_deref(),
                    assignee.as_deref(), due_date.as_deref(), status.as_deref(),
                    team.as_deref(), project.as_deref(), user_notes.as_deref(),
                    blocked_by_reason.as_deref(), task_type.as_deref(),
                ).await
            }
        },
        Commands::Decisions { command } => match command {
            DecisionsCmd::List { search, limit, offset } => {
                commands::decisions::list(search.as_deref(), limit, offset).await
            }
        },
        Commands::Intents { command } => match command {
            IntentsCmd::List { limit, offset } => {
                commands::intents::list(limit, offset).await
            }
        },
        Commands::Entity { command } => match command {
            EntityCmd::Get { ids } => commands::entity::get(ids).await,
        },
        Commands::Teams { command } => match command {
            TeamsCmd::List => commands::teams::list().await,
        },
        Commands::Projects { command } => match command {
            ProjectsCmd::List { team } => commands::projects::list(team.as_deref()).await,
            ProjectsCmd::Create { name, team, key, description } => {
                commands::projects::create(&name, &team, key.as_deref(), description.as_deref()).await
            }
        },
        Commands::Statuses { command } => match command {
            StatusesCmd::List { team } => commands::statuses::list(team.as_deref()).await,
        },
        Commands::Search { query, top_k, min_score } => {
            commands::search::search(&query, top_k, min_score).await
        }
        Commands::Context { max_tokens } => {
            commands::context::context(max_tokens).await
        }
    }
}

#[tokio::main]
async fn main() -> std::process::ExitCode {
    let cli = Cli::parse();
    let result = run(cli).await;
    match result {
        Ok(()) => std::process::ExitCode::from(0),
        Err(e) => {
            output::print_error(&e);
            e.exit_code().into()
        }
    }
}
