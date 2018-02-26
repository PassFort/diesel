use pg::Pg;
use query_builder::{AstPass, QueryFragment};
use query_builder::for_update_clause::{NoModifier, NoWait, SkipLocked};
use result::QueryResult;

impl QueryFragment<Pg> for NoModifier {
    fn walk_ast(&self, _out: AstPass<Pg>) -> QueryResult<()> {
        Ok(())
    }
}

impl QueryFragment<Pg> for SkipLocked {
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql(" SKIP LOCKED");
        Ok(())
    }
}

impl QueryFragment<Pg> for NoWait {
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql(" NOWAIT");
        Ok(())
    }
}
