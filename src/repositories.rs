use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::PgConnection;

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn find_multiple(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<Rustaceans>> {
        rustaceans::table
            .limit(limit)
            .order(rustaceans::id.desc())
            .load::<Rustaceans>(c)
    }
    pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<Rustaceans> {
        rustaceans::table
            .filter(rustaceans::id.eq(id))
            .get_result::<Rustaceans>(c)
    }
    fn last_inserted_id(c: &mut PgConnection) -> QueryResult<Option<i32>> {
        rustaceans::table
            .select(rustaceans::id)
            .order(rustaceans::id.desc())
            .first(c)
    }
    pub fn create(c: &mut PgConnection, new_rustacean: NewRustacean) -> QueryResult<Rustaceans> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .execute(c)?;
        let last_id = Self::last_inserted_id(c)?;

        match last_id {
            Some(id) => Self::find(c, id),
            None => Err(diesel::result::Error::NotFound), // Or any other appropriate error handling
        }
    }
    pub fn save(c: &mut PgConnection, id: i32, rustacean: Rustaceans) -> QueryResult<Rustaceans> {
        diesel::update(rustaceans::table.filter(rustaceans::id.eq(id)))
            .set((
                rustaceans::email.eq(rustacean.email.to_owned()),
                rustaceans::name.eq(rustacean.name.to_owned()),
            ))
            .execute(c)?;
        Self::find(c, id)
    }
    pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.filter(rustaceans::id.eq(id))).execute(c)
    }
}



pub struct CrateRepository;

impl CrateRepository {
    pub fn find_multiple(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<Crate>> {
        crates::table
            .limit(limit)
            .order(crates::id.desc())
            .load::<Crate>(c)
    }
    pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<Crate> {
        crates::table
            .filter(crates::id.eq(id))
            .get_result::<Crate>(c)
    }
    fn last_inserted_id(c: &mut PgConnection) -> QueryResult<Option<i32>> {
        crates::table
            .select(crates::id)
            .order(crates::id.desc())
            .first(c)
    }
    pub fn create(c: &mut PgConnection, new_crate: NewCrate) -> QueryResult<Crate> {
        diesel::insert_into(crates::table)
            .values(new_crate)
            .execute(c)?;
        let last_id = Self::last_inserted_id(c)?;

        match last_id {
            Some(id) => Self::find(c, id),
            None => Err(diesel::result::Error::NotFound), // Or any other appropriate error handling
        }
    }
    pub fn save(c: &mut PgConnection, id: i32, a_crate: Crate) -> QueryResult<Crate> {
        diesel::update(crates::table.filter(crates::id.eq(id)))
            .set((
                crates::code.eq(a_crate.code.to_owned()),
                crates::name.eq(a_crate.name.to_owned()),
                crates::version.eq(a_crate.version.to_owned()),
                crates::descruption.eq(a_crate.descruption.to_owned()),
                crates::rustacean_id.eq(a_crate.rustacean_id.to_owned()),
            ))
            .execute(c)?;
        Self::find(c, id)
    }
    pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates::table.filter(crates::id.eq(id))).execute(c)
    }
}
