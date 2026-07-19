mod adapter;
mod types;

pub(crate) use adapter::KrrRenderRuntimeAdapter;
pub(crate) use types::{KRR_RENDER_RUNTIME_ID, KrrMathMode, KrrRenderOutput, KrrRenderPayload};
#[cfg(test)]
pub(crate) use types::{KrrRenderRequest, KrrRenderRuntime};

#[cfg(test)]
mod test_env;
#[cfg(test)]
pub(crate) use test_env::RenderRuntimeTestEnv;
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
