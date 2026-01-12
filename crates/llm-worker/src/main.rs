use clap::Parser;
use lester_core::{SqliteStore, TagJobStatus, TaggingRules, TagSuggestion, TagSource};
use tracing::{info, warn};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value = "lester.db")]
    db_path: String,
    #[arg(long, default_value_t = 3000)]
    poll_interval_ms: u64,
    #[arg(long, default_value_t = 8)]
    batch_size: usize,
    #[arg(long, default_value_t = false)]
    once: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    let store = SqliteStore::new(args.db_path);
    store.migrate()?;

    info!("llm-worker started");
    loop {
        let jobs = store.fetch_pending_tag_jobs(args.batch_size)?;
        if jobs.is_empty() {
            if args.once {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(args.poll_interval_ms)).await;
            continue;
        }

        for job in jobs {
            store.update_tag_job_status(job.id, TagJobStatus::Running)?;
            let Some(bookmark) = store.get_bookmark(job.bookmark_id)? else {
                warn!("missing bookmark for tag job {}", job.id);
                store.update_tag_job_status(job.id, TagJobStatus::Failed)?;
                continue;
            };

            let rules = TaggingRules::new();
            let suggestions = rules
                .suggest(&bookmark.url, &bookmark.title)
                .into_iter()
                .map(|s| TagSuggestion {
                    name: s.name,
                    confidence: (s.confidence * 0.9).min(0.95),
                    source: TagSource::Llm,
                })
                .collect::<Vec<_>>();

            store.upsert_tags_for_bookmark(bookmark.id, &suggestions)?;
            store.update_tag_job_status(job.id, TagJobStatus::Done)?;
        }

        if args.once {
            break;
        }
    }

    Ok(())
}
