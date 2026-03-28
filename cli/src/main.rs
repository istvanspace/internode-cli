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
    /// Manage OI topics
    Topics {
        #[command(subcommand)]
        command: TopicsCmd,
    },
    /// Manage OI tasks
    Tasks {
        #[command(subcommand)]
        command: TasksCmd,
    },
    /// Manage OI decisions
    Decisions {
        #[command(subcommand)]
        command: DecisionsCmd,
    },
    /// View OI entity details
    Intents {
        #[command(subcommand)]
        command: IntentsCmd,
    },
    /// Manage OI teams
    Teams {
        #[command(subcommand)]
        command: TeamsCmd,
    },
    /// Manage OI projects
    Projects {
        #[command(subcommand)]
        command: ProjectsCmd,
    },
    /// Manage OI statuses
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
    /// Display teams and projects overview
    Taxonomy,
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
    /// List topics
    List {
        #[arg(long)]
        limit: Option<i64>,
        #[arg(long)]
        offset: Option<i64>,
        #[arg(long)]
        search: Option<String>,
    },
    /// Get a topic by ID
    Get {
        id: String,
        #[arg(long = "with-related")]
        with_related: bool,
    },
    /// Create a new topic
    Create {
        #[arg(long)]
        title: String,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        conclusion: Option<String>,
        #[arg(long = "conclusion-type")]
        conclusion_type: Option<String>,
    },
    /// Update an existing topic
    Update {
        id: String,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        conclusion: Option<String>,
    },
    /// Archive a topic
    Delete { id: String },
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
        #[arg(long)]
        limit: Option<i64>,
        #[arg(long)]
        offset: Option<i64>,
    },
    /// Get a task by ID
    Get {
        id: String,
        #[arg(long = "with-related")]
        with_related: bool,
    },
    /// Create a new task
    Create {
        #[arg(long)]
        title: String,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        priority: Option<String>,
        #[arg(long)]
        assignee: Option<String>,
        #[arg(long = "due-date")]
        due_date: Option<String>,
        #[arg(long)]
        team: Option<String>,
        #[arg(long)]
        project: Option<String>,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        decision: Option<String>,
        #[arg(long = "type")]
        task_type: Option<String>,
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
        #[arg(long)]
        status: Option<String>,
    },
    /// Archive a task
    Delete { id: String },
}

#[derive(Subcommand)]
enum DecisionsCmd {
    /// Get a decision by ID
    Get {
        id: String,
        #[arg(long = "with-related")]
        with_related: bool,
    },
    /// Update an existing decision
    Update {
        id: String,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        priority: Option<String>,
    },
    /// Archive a decision
    Delete { id: String },
}

#[derive(Subcommand)]
enum IntentsCmd {
    /// Get an entity by ID (returns detailed info with relations)
    Get {
        id: String,
        #[arg(long = "with-related")]
        with_related: bool,
    },
}

#[derive(Subcommand)]
enum TeamsCmd {
    /// List teams
    List,
    /// Get a team by ID
    Get { id: String },
    /// Create a new team
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        key: Option<String>,
        #[arg(long)]
        description: Option<String>,
        /// Comma-separated email addresses
        #[arg(long)]
        members: Option<String>,
    },
    /// Update an existing team
    Update {
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        key: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        members: Option<String>,
    },
    /// Archive a team
    Delete { id: String },
}

#[derive(Subcommand)]
enum ProjectsCmd {
    /// List projects
    List {
        #[arg(long)]
        team: Option<String>,
    },
    /// Get a project by ID
    Get { id: String },
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
    /// Update an existing project
    Update {
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        key: Option<String>,
        #[arg(long)]
        description: Option<String>,
    },
    /// Archive a project
    Delete { id: String },
}

#[derive(Subcommand)]
enum StatusesCmd {
    /// List statuses
    List {
        #[arg(long)]
        team: Option<String>,
    },
    /// Create a new status
    Create {
        #[arg(long)]
        name: String,
        /// Team ID (required)
        #[arg(long)]
        team: String,
        #[arg(long)]
        description: Option<String>,
    },
    /// Update an existing status
    Update {
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        description: Option<String>,
    },
    /// Archive a status
    Delete { id: String },
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
            TopicsCmd::List { limit, offset, search } => {
                commands::topics::list(limit, offset, search.as_deref()).await
            }
            TopicsCmd::Get { id, with_related } => {
                commands::topics::get(&id, with_related).await
            }
            TopicsCmd::Create { title, description, conclusion, conclusion_type } => {
                commands::topics::create(&title, description.as_deref(), conclusion.as_deref(), conclusion_type.as_deref()).await
            }
            TopicsCmd::Update { id, title, description, conclusion } => {
                commands::topics::update(&id, title.as_deref(), description.as_deref(), conclusion.as_deref()).await
            }
            TopicsCmd::Delete { id } => commands::topics::delete(&id).await,
        },
        Commands::Tasks { command } => match command {
            TasksCmd::List { team, project, status, assignee, priority, search, limit, offset } => {
                commands::tasks::list(team.as_deref(), project.as_deref(), status.as_deref(), assignee.as_deref(), priority.as_deref(), search.as_deref(), limit, offset).await
            }
            TasksCmd::Get { id, with_related } => {
                commands::tasks::get(&id, with_related).await
            }
            TasksCmd::Create { title, description, priority, assignee, due_date, team, project, status, decision, task_type } => {
                commands::tasks::create(&title, description.as_deref(), priority.as_deref(), assignee.as_deref(), due_date.as_deref(), team.as_deref(), project.as_deref(), status.as_deref(), decision.as_deref(), task_type.as_deref()).await
            }
            TasksCmd::Update { id, title, description, priority, assignee, due_date, status } => {
                commands::tasks::update(&id, title.as_deref(), description.as_deref(), priority.as_deref(), assignee.as_deref(), due_date.as_deref(), status.as_deref()).await
            }
            TasksCmd::Delete { id } => commands::tasks::delete(&id).await,
        },
        Commands::Decisions { command } => match command {
            DecisionsCmd::Get { id, with_related } => {
                commands::decisions::get(&id, with_related).await
            }
            DecisionsCmd::Update { id, title, description, status, priority } => {
                commands::decisions::update(&id, title.as_deref(), description.as_deref(), status.as_deref(), priority.as_deref()).await
            }
            DecisionsCmd::Delete { id } => commands::decisions::delete(&id).await,
        },
        Commands::Intents { command } => match command {
            IntentsCmd::Get { id, with_related } => {
                commands::intents::get(&id, with_related).await
            }
        },
        Commands::Teams { command } => match command {
            TeamsCmd::List => commands::teams::list().await,
            TeamsCmd::Get { id } => commands::teams::get(&id).await,
            TeamsCmd::Create { name, key, description, members } => {
                commands::teams::create(&name, key.as_deref(), description.as_deref(), members.as_deref()).await
            }
            TeamsCmd::Update { id, name, key, description, members } => {
                commands::teams::update(&id, name.as_deref(), key.as_deref(), description.as_deref(), members.as_deref()).await
            }
            TeamsCmd::Delete { id } => commands::teams::delete(&id).await,
        },
        Commands::Projects { command } => match command {
            ProjectsCmd::List { team } => commands::projects::list(team.as_deref()).await,
            ProjectsCmd::Get { id } => commands::projects::get(&id).await,
            ProjectsCmd::Create { name, team, key, description } => {
                commands::projects::create(&name, &team, key.as_deref(), description.as_deref()).await
            }
            ProjectsCmd::Update { id, name, key, description } => {
                commands::projects::update(&id, name.as_deref(), key.as_deref(), description.as_deref()).await
            }
            ProjectsCmd::Delete { id } => commands::projects::delete(&id).await,
        },
        Commands::Statuses { command } => match command {
            StatusesCmd::List { team } => commands::statuses::list(team.as_deref()).await,
            StatusesCmd::Create { name, team, description } => {
                commands::statuses::create(&name, &team, description.as_deref()).await
            }
            StatusesCmd::Update { id, name, description } => {
                commands::statuses::update(&id, name.as_deref(), description.as_deref()).await
            }
            StatusesCmd::Delete { id } => commands::statuses::delete(&id).await,
        },
        Commands::Search { query, top_k, min_score } => {
            commands::search::search(&query, top_k, min_score).await
        }
        Commands::Taxonomy => commands::taxonomy::taxonomy().await,
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
