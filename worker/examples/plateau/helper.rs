use std::{env, fs, path::Path, sync::Arc};

use directories::ProjectDirs;
use tracing::Level;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;
use yaml_include::Transformer;

use reearth_flow_action_log::factory::{create_root_logger, LoggerFactory};
use reearth_flow_common::uri::Uri;
use reearth_flow_state::State;
use reearth_flow_storage::resolve::StorageResolver;
use reearth_flow_workflow::workflow::Workflow;
use reearth_flow_workflow_runner::dag::DagExecutor;

pub(crate) fn init_execute_runner(workflow: &str) -> DagExecutor {
    env::set_var("RAYON_NUM_THREADS", "6");
    setup_logging_and_tracing();
    let job_id = uuid::Uuid::new_v4();
    let dataframe_state_uri = {
        let p = ProjectDirs::from("reearth", "flow", "worker").unwrap();
        let p = p.data_dir().to_str().unwrap();
        let p = format!("{}/dataframe/{}", p, job_id);
        let _ = fs::create_dir_all(Path::new(p.as_str()));
        Uri::for_test(format!("file://{}", p).as_str())
    };
    let action_log_uri = {
        let p = ProjectDirs::from("reearth", "flow", "worker").unwrap();
        let p = p.data_dir().to_str().unwrap();
        let p = format!("{}/action-log/{}", p, job_id);
        let _ = fs::create_dir_all(Path::new(p.as_str()));
        Uri::for_test(format!("file://{}", p).as_str())
    };

    let storage_resolver = Arc::new(StorageResolver::new());
    let state = Arc::new(State::new(&dataframe_state_uri, &storage_resolver).unwrap());
    let workflow = create_workflow(workflow);
    let log_factory = Arc::new(LoggerFactory::new(
        create_root_logger(action_log_uri.path()),
        action_log_uri.path(),
    ));
    DagExecutor::new(job_id, &workflow, storage_resolver, state, log_factory).unwrap()
}

pub fn create_workflow(workflow: &str) -> Workflow {
    let current_dir = env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
        .replace("examples", "");
    let current_dir = Path::new(&current_dir);
    let absolute_path = fs::canonicalize(
        current_dir
            .join("examples/plateau/testdata/workflow")
            .join(workflow),
    );
    let path = absolute_path.unwrap();
    let yaml = Transformer::new(path, false).unwrap();
    let yaml = yaml.to_string();
    Workflow::try_from_str(yaml.as_str()).unwrap()
}

pub fn setup_logging_and_tracing() {
    let env_filter = EnvFilter::builder()
        .with_default_directive(Level::INFO.into())
        .from_env_lossy()
        .add_directive("opendal=error".parse().unwrap());
    let registry = tracing_subscriber::registry().with(env_filter);
    let event_format = tracing_subscriber::fmt::format()
        .with_target(true)
        .with_timer(UtcTime::new(
            time::format_description::parse(
                "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]Z",
            )
            .expect("Time format invalid."),
        ));
    let _ = registry
        .with(
            tracing_subscriber::fmt::layer()
                .event_format(event_format)
                .with_ansi(true),
        )
        .try_init();
}
