from mysql.connector import Error as MySQLError

from database import get_pool


def add_user_to_db(user_data):
    conn = None
    cursor = None
    try:
        conn = get_pool().get_connection()
        cursor = conn.cursor(dictionary=True)
        insert_query = """
        INSERT INTO Users (id, full_name, team, creation_date, active, admin)
        VALUES (%s, %s, %s, %s, %s, %s)
        """
        cursor.execute(insert_query, (
            user_data['id'],
            user_data['full_name'],
            user_data['team'],
            user_data['creation_date'],
            user_data['active'],
            user_data['admin']
        ))
        conn.commit()
        return True
    except MySQLError as err:
        print(f"Error: {err}")
        return False
    finally:
        if cursor:
            cursor.close()
        if conn:
            conn.close()

def set_tables() -> bool:
    conn = None
    cursor = None
    print('Setting up database tables...')    
    users_table_query = open('users_table.sql').read()
    user_data_table_query = open('user_data_table.sql').read()
    try:
        conn = get_pool().get_connection()
        cursor = conn.cursor(dictionary=True)
        cursor.execute(users_table_query)
        cursor.execute(user_data_table_query)
        conn.commit()    
        return True
    except Exception as why:
        print(f'Error! Something went wrong while creating tables!\n{why}')
        return False
    finally:
        if cursor:
            cursor.close()
        if conn:
            conn.close()