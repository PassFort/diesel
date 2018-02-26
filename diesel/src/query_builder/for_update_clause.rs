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
pub struct ForUpdateClause<Modifier = NoModifier> {
    modifier: Modifier,
}

impl<Modifier> ForUpdateClause<Modifier> {
    pub(crate) fn new(modifier: Modifier) -> Self {
        ForUpdateClause { modifier }
    }
}

impl<DB: Backend, M: QueryFragment<DB>> QueryFragment<DB> for ForUpdateClause<M> {
    fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
        out.push_sql(" FOR UPDATE");
        self.modifier.walk_ast(out)
    }
}

#[derive(Debug, Clone, Copy, QueryId)]
pub struct NoModifier;

#[derive(Debug, Clone, Copy, QueryId)]
pub struct SkipLocked;

#[derive(Debug, Clone, Copy, QueryId)]
pub struct NoWait;
