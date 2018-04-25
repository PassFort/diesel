use mysql::Mysql;
use query_builder::{AstPass, QueryFragment};
use query_builder::locking_clause::{NoModifier, NoWait, SkipLocked};
use result::QueryResult;

impl QueryFragment<Mysql> for NoModifier {
    fn walk_ast(&self, _out: AstPass<Mysql>) -> QueryResult<()> {
        Ok(())
    }
}

impl QueryFragment<Mysql> for SkipLocked {
    fn walk_ast(&self, mut out: AstPass<Mysql>) -> QueryResult<()> {
        out.push_sql(" SKIP LOCKED");
        Ok(())
    }
}

impl QueryFragment<Mysql> for NoWait {
    fn walk_ast(&self, mut out: AstPass<Mysql>) -> QueryResult<()> {
        out.push_sql(" NOWAIT");
        Ok(())
    }
}
