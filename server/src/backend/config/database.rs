use mysql::prelude::*;
use mysql::*;

pub fn establish_connection() -> PooledConn {
    let url = "mysql://root:12345678@localhost:3306/shennong";
    let pool = Pool::new(url).expect("Failed to create pool.");
    pool.get_conn()
        .expect("Failed to get connection from pool.")
}
