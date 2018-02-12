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
pub struct ForUpdateClause<Modifier> {
    pub(crate) modifier: Modifier,
}

#[derive(Debug, Clone, Copy, QueryId)]
pub struct NoModifier;

#[derive(Debug, Clone, Copy, QueryId)]
pub struct SkipLockedModifier;

#[derive(Debug, Clone, Copy, QueryId)]
pub struct NoWaitModifier;
