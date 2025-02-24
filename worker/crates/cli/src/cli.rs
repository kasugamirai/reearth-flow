use std::env;

use clap::{ArgMatches, Command};
use tracing::Level;

use crate::dot::{build_dot_command, DotCliCommand};
use crate::run::{build_run_command, RunCliCommand};
use crate::schema_action::{build_schema_action_command, SchemaActionCliCommand};
use crate::schema_workflow::{build_schema_workflow_command, SchemaWorkflowCliCommand};

pub fn build_cli() -> Command {
    Command::new("Re:Earth Flow")
        .subcommand(build_run_command().display_order(1))
        .subcommand(build_dot_command().display_order(2))
        .subcommand(build_schema_action_command().display_order(3))
        .subcommand(build_schema_workflow_command().display_order(4))
        .arg_required_else_help(true)
        .disable_help_subcommand(true)
        .subcommand_required(true)
}

#[derive(Debug, PartialEq)]
pub enum CliCommand {
    Run(RunCliCommand),
    Dot(DotCliCommand),
    SchemaAction(SchemaActionCliCommand),
    SchemaWorkflow(SchemaWorkflowCliCommand),
}

impl CliCommand {
    pub fn default_log_level(&self) -> Level {
        let env_level = env::var("RUST_LOG")
            .ok()
            .and_then(|s| s.parse::<Level>().ok());
        env_level.unwrap_or(match self {
            CliCommand::Run(_) => Level::INFO,
            CliCommand::Dot(_) => Level::WARN,
            CliCommand::SchemaAction(_) => Level::WARN,
            CliCommand::SchemaWorkflow(_) => Level::WARN,
        })
    }

    pub fn parse_cli_args(mut matches: ArgMatches) -> crate::Result<Self> {
        let (subcommand, submatches) = matches
            .remove_subcommand()
            .ok_or(crate::Error::parse("missing subcommand"))?;
        match subcommand.as_str() {
            "run" => RunCliCommand::parse_cli_args(submatches).map(CliCommand::Run),
            "dot" => DotCliCommand::parse_cli_args(submatches).map(CliCommand::Dot),
            "schema-action" => Ok(CliCommand::SchemaAction(SchemaActionCliCommand)),
            "schema-workflow" => Ok(CliCommand::SchemaWorkflow(SchemaWorkflowCliCommand)),
            _ => Err(crate::Error::unknown_command(subcommand)),
        }
    }

    pub fn execute(self) -> crate::Result<()> {
        match self {
            CliCommand::Run(subcommand) => subcommand.execute(),
            CliCommand::Dot(subcommand) => subcommand.execute(),
            CliCommand::SchemaAction(subcommand) => subcommand.execute(),
            CliCommand::SchemaWorkflow(subcommand) => subcommand.execute(),
        }
    }
}
