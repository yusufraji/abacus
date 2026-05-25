use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqliteConnection};

use crate::{domain::identifier::id, repository::user_repo::UserId};

id!(SessionEntryId);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, sqlx::Type)]
enum LoggedOutReason {
    TimedOut,
    LoggedOutByUser,
}
/// A session object, corresponds to a row in the sessions table
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, FromRow)]
#[serde(deny_unknown_fields)]
pub(crate) struct SessionEntry {
    id: SessionEntryId,
    session_key: String,
    user_id: UserId,
    user_agent: String,
    ip_address: String,
    created_at: DateTime<Utc>,
    logged_out_at: Option<DateTime<Utc>>,
    logged_out_reason: Option<LoggedOutReason>,
}

pub(crate) struct ActiveSession {
    session_entry_id: SessionEntryId,
    expires_at: DateTime<Utc>,
}

impl SessionEntry {
    pub(crate) fn new() -> Self {
        Self {
            id,
            session_key,
            user_id,
            user_agent,
            ip_address,
            created_at,
            logged_out_at: None,
            logged_out_reason: None,
        }
    }

    /// Get the session user id
    pub(crate) fn user_id(&self) -> UserId {
        self.user_id
    }

    /// Get the session key
    pub(crate) fn session_key(&self) -> &str {
        &self.session_key
    }

    /// Get the age of a session
    pub(crate) fn duration(&self) -> Duration {
        Utc::now()
            .signed_duration_since(self.created_at)
            .to_std()
            .unwrap_or_default()
    }
}

impl ActiveSession {
    fn new(session_entry_id: SessionEntryId, expires_at: DateTime<Utc>) -> Self {
        Self {
            session_entry_id,
            expires_at,
        }
    }

    pub(crate) fn session_entry_id(&self) -> SessionEntryId {
        self.session_entry_id
    }

    /// Get the session expiration time
    pub(crate) fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }
}

/// Save a session, note this converts any i64 timestamps to i64
pub(crate) async fn create(
    conn: &mut SqliteConnection,
    session_key: String,
    user_id: UserId,
    user_agent: String,
    ip_address: String,
    created_at: DateTime<Utc>,
) -> Result<ActiveSession, sqlx::Error> {
    let now = Utc::now();
    let no_datetime: Option<DateTime<Utc>> = None;
    let session_entry= 
{
    {
        #[allow(clippy::all)]
        {
            use ::sqlx::Arguments as _;
            let arg0 = &(session_key);
            let arg1 = &(user_id);
            let arg2 = &(user_agent);
            let arg3 = &(ip_address);
            let arg4 = &(now);
            let arg5 = &(no_datetime.clone());
            let arg6 = &(no_datetime);
            let mut query_args =
                <sqlx::sqlite::Sqlite as ::sqlx::database::Database>::Arguments::<'_>::default(
                );
            query_args.reserve(
                7usize,
                0 + ::sqlx::encode::Encode::<sqlx::sqlite::Sqlite>::size_hint(arg0)
                    + ::sqlx::encode::Encode::<sqlx::sqlite::Sqlite>::size_hint(arg1)
                    + ::sqlx::encode::Encode::<sqlx::sqlite::Sqlite>::size_hint(arg2)
                    + ::sqlx::encode::Encode::<sqlx::sqlite::Sqlite>::size_hint(arg3)
                    + ::sqlx::encode::Encode::<sqlx::sqlite::Sqlite>::size_hint(arg4)
                    + ::sqlx::encode::Encode::<sqlx::sqlite::Sqlite>::size_hint(arg5)
                    + ::sqlx::encode::Encode::<sqlx::sqlite::Sqlite>::size_hint(arg6),
            );
            let query_args =
                ::core::result::Result::<_, ::sqlx::error::BoxDynError>::Ok(query_args)
                    .and_then(move |mut query_args| {
                        query_args.add(arg0).map(move |()| query_args)
                    })
                    .and_then(move |mut query_args| {
                        query_args.add(arg1).map(move |()| query_args)
                    })
                    .and_then(move |mut query_args| {
                        query_args.add(arg2).map(move |()| query_args)
                    })
                    .and_then(move |mut query_args| {
                        query_args.add(arg3).map(move |()| query_args)
                    })
                    .and_then(move |mut query_args| {
                        query_args.add(arg4).map(move |()| query_args)
                    })
                    .and_then(move |mut query_args| {
                        query_args.add(arg5).map(move |()| query_args)
                    })
                    .and_then(move |mut query_args| {
                        query_args.add(arg6).map(move |()| query_args)
                    });
            ::sqlx::__query_with_result:: <sqlx::sqlite::Sqlite,_>("INSERT INTO session_entry (session_key, user_id, user_agent, ip_address, created_at, logged_out_at, logged_out_reason)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        RETURNING
            id as \"id: SessionEntryId\",
            session_key,
            user_id as \"user_id: UserId\",
            user_agent,
            ip_address,
            created_at as \"created_at: _\",
            logged_out_at as \"logged_out_at: DateTime<Utc>\",
            logged_out_reason as \"logged_out_reason: LoggedOutReason\"
        ",query_args).try_map(|row: sqlx::sqlite::SqliteRow|{
            use::sqlx::Row as _;
            #[allow(non_snake_case)]
            let sqlx_query_as_id = row.try_get_unchecked:: <SessionEntryId,_>(0usize)? .into();
            #[allow(non_snake_case)]
            let sqlx_query_as_session_key = row.try_get_unchecked:: <String,_>(1usize)? .into();
            #[allow(non_snake_case)]
            let sqlx_query_as_user_id = row.try_get_unchecked:: <UserId,_>(2usize)? .into();
            #[allow(non_snake_case)]
            let sqlx_query_as_user_agent = row.try_get_unchecked:: <String,_>(3usize)? .into();
            #[allow(non_snake_case)]
            let sqlx_query_as_ip_address = row.try_get_unchecked:: <String,_>(4usize)? .into();
            #[allow(non_snake_case)]
            let sqlx_query_as_created_at = row.try_get(5usize)? ;
            #[allow(non_snake_case)]
            let sqlx_query_as_logged_out_at = row.try_get_unchecked:: < ::std::option::Option<DateTime<Utc> > ,_>(6usize)? .into();
            #[allow(non_snake_case)]
            let sqlx_query_as_logged_out_reason = row.try_get_unchecked:: < ::std::option::Option<LoggedOutReason> ,_>(7usize)? .into();
            ::std::result::Result::Ok(SessionEntry {
                r#id: sqlx_query_as_id
,r#session_key: sqlx_query_as_session_key
,r#user_id: sqlx_query_as_user_id
,r#user_agent: sqlx_query_as_user_agent
,r#ip_address: sqlx_query_as_ip_address
,r#created_at: sqlx_query_as_created_at
,r#logged_out_at: sqlx_query_as_logged_out_at
,r#logged_out_reason: sqlx_query_as_logged_out_reason
            })
        })
        }
    }
}
        .fetch_one(conn)
        .await?;
    let saved_session = sqlx::query_as!(
        ActiveSession,
        r#"INSERT INTO active_sessions (session_entry_id, expires_at)
        VALUES (?, ?)
        RETURNING
            session_entry_id as "session_entry_id: SessionEntryId",
            expires_at as "expires_at: _"
        "#,
        session_entry.id,
        user_id
    )
    .fetch_one(conn)
    .await?;

    Ok(saved_session)
}

#[derive(Debug)]
pub(crate) struct SessionIdentifier {
    pub session_key: String,
    pub user_agent: String,
    pub ip_address: String,
}

/// Get a session by its key and validate user agent and IP address
pub(crate) async fn get_by_identifier(
    conn: &mut SqliteConnection,
    session: &SessionIdentifier,
) -> Result<Option<Session>, sqlx::Error> {
    let now = Utc::now();

    let session: Option<Session> = sqlx::query_as!(
        Session,
        r#"
        SELECT
            session_key,
            user_id as "user_id: UserId",
            user_agent,
            ip_address,
            expires_at as "expires_at: _",
            created_at as "created_at: _"
        FROM sessions
        WHERE session_key = ?
        AND user_agent = ?
        AND ip_address = ?
        AND expires_at > ?
        "#,
        session.session_key,
        session.user_agent,
        session.ip_address,
        now
    )
    .fetch_optional(conn)
    .await?;

    Ok(session)
}

/// Get a session by its key
pub(crate) async fn get_by_key(
    conn: &mut SqliteConnection,
    session_key: &str,
) -> Result<Option<Session>, sqlx::Error> {
    let now = Utc::now();
    let session: Option<Session> = sqlx::query_as!(
        Session,
        r#"
        SELECT
            session_key,
            user_id as "user_id: UserId",
            user_agent,
            ip_address,
            expires_at as "expires_at: _",
            created_at as "created_at: _"
        FROM sessions
        WHERE session_key = ?
        AND expires_at > ?
        "#,
        session_key,
        now
    )
    .fetch_optional(conn)
    .await?;

    Ok(session)
}

/// Delete a session by its key
pub(crate) async fn delete(
    conn: &mut SqliteConnection,
    session_key: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM sessions WHERE session_key = ?", session_key)
        .execute(conn)
        .await?;

    Ok(())
}

/// Delete a session for a certain user
pub(crate) async fn delete_user_session(
    conn: &mut SqliteConnection,
    user_id: UserId,
) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM sessions WHERE user_id = ?", user_id)
        .execute(conn)
        .await?;

    Ok(())
}

/// Delete all sessions that have expired
pub(crate) async fn delete_expired_sessions(
    conn: &mut SqliteConnection,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM sessions WHERE expires_at <= ?")
        .bind(Utc::now())
        .execute(conn)
        .await?;

    Ok(())
}

/// Count the number of active sessions
pub(crate) async fn count(conn: &mut SqliteConnection) -> Result<u32, sqlx::Error> {
    let now = Utc::now();
    let count = sqlx::query_scalar!(
        r#"SELECT COUNT(*) AS "count: u32" FROM sessions WHERE expires_at > ?"#,
        now
    )
    .fetch_one(conn)
    .await?;

    Ok(count)
}

pub(crate) async fn extend_session(
    conn: &mut SqliteConnection,
    session: &Session,
    expires_at: DateTime<Utc>,
) -> Result<Session, sqlx::Error> {
    let session_key = session.session_key();

    let session = sqlx::query_as!(
        Session,
        r#"
        UPDATE sessions
        SET expires_at = ?
        WHERE session_key = ?
        RETURNING
            session_key,
            user_id as "user_id: UserId",
            user_agent,
            ip_address,
            expires_at as "expires_at: _",
            created_at as "created_at: _"
        "#,
        expires_at,
        session_key
    )
    .fetch_one(conn)
    .await?;

    Ok(session)
}

#[cfg(test)]
mod tests {
    use chrono::TimeDelta;
    use sqlx::SqlitePool;
    use test_log::test;

    use super::*;
    use crate::repository::user_repo::UserId;

    const TEST_USER_AGENT: &str = "TestAgent/1.0";
    const TEST_IP_ADDRESS: &str = "0.0.0.0";

    #[test(sqlx::test(fixtures("../../fixtures/users.sql")))]
    async fn test_create_and_get_session(pool: SqlitePool) {
        let mut conn = pool.acquire().await.unwrap();
        let session = Session::create(
            UserId::from(1),
            TEST_USER_AGENT,
            TEST_IP_ADDRESS,
            TimeDelta::seconds(60),
        );
        create(&mut conn, &session).await.unwrap();

        let session_from_db = super::get_by_key(&mut conn, &session.session_key)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(session, session_from_db);
    }

    #[test(sqlx::test(fixtures("../../fixtures/users.sql")))]
    async fn test_delete_session(pool: SqlitePool) {
        let mut conn = pool.acquire().await.unwrap();
        let session = Session::create(
            UserId::from(1),
            TEST_USER_AGENT,
            TEST_IP_ADDRESS,
            TimeDelta::seconds(60),
        );
        create(&mut conn, &session).await.unwrap();

        let session_from_db = super::get_by_key(&mut conn, &session.session_key)
            .await
            .unwrap();
        assert_eq!(session_from_db, Some(session.clone()));

        super::delete(&mut conn, &session.session_key)
            .await
            .unwrap();

        let session_from_db = super::get_by_key(&mut conn, session.session_key())
            .await
            .unwrap();

        assert_eq!(None, session_from_db);
    }

    #[test(sqlx::test(fixtures("../../fixtures/users.sql")))]
    async fn test_delete_old_sessions(pool: SqlitePool) {
        let mut conn = pool.acquire().await.unwrap();
        let session = Session::create(
            UserId::from(1),
            TEST_USER_AGENT,
            TEST_IP_ADDRESS,
            TimeDelta::seconds(0),
        );
        create(&mut conn, &session).await.unwrap();

        delete_expired_sessions(&mut conn).await.unwrap();

        let session_from_db = super::get_by_key(&mut conn, session.session_key())
            .await
            .unwrap();

        assert_eq!(None, session_from_db);
    }

    #[test(sqlx::test(fixtures("../../fixtures/users.sql")))]
    async fn test_session_count(pool: SqlitePool) {
        let mut conn = pool.acquire().await.unwrap();
        let active_session1 = Session::create(
            UserId::from(1),
            TEST_USER_AGENT,
            TEST_IP_ADDRESS,
            TimeDelta::seconds(60),
        );
        create(&mut conn, &active_session1).await.unwrap();

        let active_session2 = Session::create(
            UserId::from(2),
            TEST_USER_AGENT,
            TEST_IP_ADDRESS,
            TimeDelta::seconds(120),
        );
        create(&mut conn, &active_session2).await.unwrap();

        let expired_session = Session::create(
            UserId::from(2),
            TEST_USER_AGENT,
            TEST_IP_ADDRESS,
            TimeDelta::seconds(0),
        );
        create(&mut conn, &expired_session).await.unwrap();

        assert_eq!(2, super::count(&mut conn).await.unwrap());
    }
}
