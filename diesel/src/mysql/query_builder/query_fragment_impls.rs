use mysql::Mysql;
use query_builder::{AstPass, QueryFragment};
use query_builder::for_update_clause::{ForUpdateClause, NoModifier};
use result::QueryResult;

impl QueryFragment<Mysql> for ForUpdateClause<NoModifier> {
    fn walk_ast(&self, mut out: AstPass<Mysql>) -> QueryResult<()> {
        out.push_sql(" FOR UPDATE");
        Ok(())
    }
}
