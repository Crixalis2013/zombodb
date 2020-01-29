mod dsl {
    use crate::zdbquery::ZDBQuery;
    use pgx::*;

    #[pg_extern]
    pub(super) fn limit(limit: i64, query: ZDBQuery) -> ZDBQuery {
        query.set_limit(Some(limit as u64))
    }
}

mod tests {
    use crate::query_dsl::limit::dsl::limit;
    use crate::zdbquery::ZDBQuery;
    use pgx::*;
    use serde_json::*;

    #[test]
    fn make_idea_happy() {}

    #[pg_test]
    fn test_limit() {
        let zdbquery = limit(100, ZDBQuery::new_with_query_string("test"));

        assert_eq!(
            zdbquery.into_value(),
            json! {
                { "limit": 100, "query_dsl": { "query_string": { "query": "test" } } }
            }
        )
    }
}
