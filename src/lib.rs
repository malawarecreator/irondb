pub mod db;


#[cfg(test)]
mod tests {
    use crate::db::dblock::Dblock;
    use crate::db::database::Db;

    #[test]
    fn test_dblock_fmt() {
        let block = Dblock::new("hello", "world");
        println!("{}", block);
    }
    
    #[test]
    fn test_db_fmt() {
        let mut db: Db<_, _> = Db::new("testdb");
        db.save(Dblock::new(1, 2));
        db.getbp(1);

    }

    


    
    
}