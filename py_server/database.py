import os
from mysql.connector import pooling

pool = None

def init_pool():
    global pool
    pool = pooling.MySQLConnectionPool(
        pool_name="mypool",
        pool_size=5,
        pool_reset_session=True,
        host=os.getenv('DB_HOST', 'localhost'),
        user=os.getenv('DB_USER'),
        password=os.getenv('DB_PASSWORD'),
        database=os.getenv('DB_NAME'),
    )

def get_pool():
    if pool is None:
        raise Exception("Database pool is not initialized. Call init_pool() first.")
    return pool

