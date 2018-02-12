use pg::Pg;
use query_builder::{AstPass, QueryFragment};
use query_builder::for_update_clause::{ForUpdateClause, NoModifier, NoWaitModifier,
                                       SkipLockedModifier};
use result::QueryResult;

impl QueryFragment<Pg> for ForUpdateClause<NoModifier> {
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql(" FOR UPDATE");
        Ok(())
    }
}

impl QueryFragment<Pg> for ForUpdateClause<SkipLockedModifier> {
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql(" FOR UPDATE SKIP LOCKED");
        Ok(())
    }
}

impl QueryFragment<Pg> for ForUpdateClause<NoWaitModifier> {
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql(" FOR UPDATE NOWAIT");
        Ok(())
    }
}
