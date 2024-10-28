#![allow(dead_code)]
extern crate memorix_client_redis;


#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[memorix_client_redis::serialization]
#[derive(Clone, PartialEq, std::fmt::Debug)]
pub enum System {
    NODE,
    DENO,
    BUN,
    RUST,
    PYTHON,
}

#[derive(Clone)]
#[allow(non_snake_case)]
pub struct MemorixTask {
    pub pass_ball: memorix_client_redis::MemorixTaskItem<System, u64, memorix_client_redis::Expose, memorix_client_redis::Expose, memorix_client_redis::Expose, memorix_client_redis::Expose>,
}

impl MemorixTask {
    fn new(memorix_base: memorix_client_redis::MemorixBase) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        Ok(Self {
            pass_ball: memorix_client_redis::MemorixTaskItem::new(
                memorix_base.clone(),
                "pass_ball".to_string(),
                Some(memorix_client_redis::MemorixTaskOptions {
                    queue_type: None,
                }),
            )?,
        })
    }
}

#[derive(Clone)]
#[allow(non_snake_case)]
pub struct Memorix {
    pub task: MemorixTask,
}

const MEMORIX_NAMESPACE_NAME_TREE: &[&str] = &[];

impl Memorix {
    pub async fn new() -> Result<Memorix, Box<dyn std::error::Error + Sync + Send>> {
        let memorix_base = memorix_client_redis::MemorixBase::new(
            &std::env::var("REDIS_URL").expect("missing environment variable REDIS_URL"),
            MEMORIX_NAMESPACE_NAME_TREE,
        )
        .await?;
        Ok(Self {
            task: MemorixTask::new(memorix_base.clone())?,
        })
    }
}
