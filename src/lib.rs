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
        let mut db: Db<i32, i32> = Db::new("testdb");
        db.save(Dblock::new(1, 2));
        db.getbp(1);

    }

    #[test]
    fn test_db_getb() {
        let mut db: Db<i32, i32> = Db::new("testdb");
        db.save(Dblock::new(1, 2));
        let block = db.getb(1).unwrap();
        assert_eq!(block.data, 2);
        assert_eq!(block.key, 1);

    }

    


    
    
}