use mysql::*;
use mysql::prelude::*;

pub fn establish_connection() -> PooledConn {
    let url = "mysql://username:password@localhost:3306/shennong";
    let pool = Pool::new(url).expect("Failed to create pool.");
    pool.get_conn().expect("Failed to get connection from pool.")
}