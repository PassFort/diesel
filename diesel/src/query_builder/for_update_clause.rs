use backend::Backend;
use query_builder::{AstPass, QueryFragment};
use result::QueryResult;

#[derive(Debug, Clone, Copy, QueryId)]
pub struct NoForUpdateClause;

impl<DB: Backend> QueryFragment<DB> for NoForUpdateClause {
    fn walk_ast(&self, _: AstPass<DB>) -> QueryResult<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, QueryId)]
pub struct ForUpdateClause<LockMode = ForUpdate, Modifier = NoModifier> {
    pub(crate) lock_mode: LockMode,
    modifier: Modifier,
}

impl<LockMode, Modifier> ForUpdateClause<LockMode, Modifier> {
    pub(crate) fn new(lock_mode: LockMode, modifier: Modifier) -> Self {
        ForUpdateClause { lock_mode, modifier }
    }
}

impl<DB: Backend, L: QueryFragment<DB>, M: QueryFragment<DB>> QueryFragment<DB> for ForUpdateClause<L, M> {
    fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
        self.lock_mode.walk_ast(out.reborrow())?;
        self.modifier.walk_ast(out.reborrow())
    }
}

#[derive(Debug, Clone, Copy, QueryId)]
pub struct ForUpdate;

#[derive(Debug, Clone, Copy, QueryId)]
pub struct ForNoKeyUpdate;

#[derive(Debug, Clone, Copy, QueryId)]
pub struct ForShare;

#[derive(Debug, Clone, Copy, QueryId)]
pub struct ForKeyShare;

#[derive(Debug, Clone, Copy, QueryId)]
pub struct NoModifier;

#[derive(Debug, Clone, Copy, QueryId)]
pub struct SkipLocked;

#[derive(Debug, Clone, Copy, QueryId)]
pub struct NoWait;
