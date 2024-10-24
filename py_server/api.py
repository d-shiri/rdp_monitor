from flask import Flask, request, Blueprint, jsonify
from mysql.connector import pooling, Error as MySQLError
from pydantic import ValidationError

from models.user import User
from funcs import add_user_to_db
from database import get_pool

api_bp = Blueprint('api', __name__)

@api_bp.route('/api/create_user', methods=['POST'])
def create_user():
    try:
        # Parse and validate incoming JSON data
        user_data = request.json
        user = User(**user_data)
        success = add_user_to_db(user.dict())
        if success:
            return jsonify({"message": "User created successfully!"}), 201
        else:
            return jsonify({"error": "Failed to create user in database."}), 500
    except ValidationError as e:
        return jsonify({"error": "Invalid user data", "details": e.errors()}), 400
    except Exception as e:
        return jsonify({"error": f"An error occurred while creating a new user: {str(e)}"}), 500

 
@api_bp.route('/api/get_user_info', methods=['GET'])
def get_user_info():
    conn = None
    cursor = None
    # Get the 'id' from query parameters
    user_id = request.args.get('id')
    if not user_id:
        return jsonify({"error": "Missing or invalid 'id' parameter"}), 400
    try:
        conn = get_pool().get_connection()
        cursor = conn.cursor(dictionary=True)
        query = "SELECT * FROM Users WHERE id = %s"
        cursor.execute(query, (user_id,))
        rows = cursor.fetchall()
        if not rows:
            return jsonify({"error": "User not found"}), 404
        return jsonify(rows)
    except MySQLError as e:
        app.logger.error(f"Database error: {e}")
        return jsonify({"error": "Failed to fetch data"}), 500
    except Exception as e:
        app.logger.error(f"Unexpected error: {e}")
        return jsonify({"error": "An unexpected error occurred"}), 500
    finally:
        if cursor:
            cursor.close()
        if conn:
            conn.close()

@api_bp.route('/api/get_user_history', methods=['GET'])
def get_user_history():
    conn = None
    cursor = None
    # Get the 'id' and 'day' from query parameters
    user_id = request.args.get('id')
    day = request.args.get('day')
    if not user_id:
        return jsonify({"error": "Missing or invalid 'id' parameter"}), 400
    if not day or not day.isdigit() or not (1 <= int(day) <= 365):
        return jsonify({"error": "Missing or invalid 'day' parameter\n 1<day<365"}), 400
    day = int(day)
    try:
        conn = get_pool().get_connection()
        cursor = conn.cursor(dictionary=True)
        query = """
        SELECT * FROM UserData WHERE user_id = %s
        AND rdp_start_time >= NOW() - INTERVAL %s DAY 
        """
        cursor.execute(query, (user_id, day,))
        rows = cursor.fetchall()
        if not rows:
            return jsonify({"error": "No user history"}), 404
        return jsonify(rows)
    except MySQLError as e:
        app.logger.error(f"Database error: {e}")
        return jsonify({"error": "Failed to fetch data"}), 500
    except Exception as e:
        app.logger.error(f"Unexpected error: {e}")
        return jsonify({"error": "An unexpected error occurred"}), 500
    finally:
        if cursor:
            cursor.close()
        if conn:
            conn.close()



@api_bp.route('/api/get_live_data', methods=['GET'])
def get_live_data():
    conn = None
    cursor = None
    try:
        conn = get_pool().get_connection()
        cursor = conn.cursor(dictionary=True)
        query = """
            SELECT full_name, remote_pc 
            FROM Users 
            INNER JOIN UserData on Users.id = UserData.user_id 
            WHERE rdp_start_time = rdp_end_time
            AND rdp_start_time >= NOW() - INTERVAL 12 HOUR;
        """
        cursor.execute(query)
        rows = cursor.fetchall()
        return jsonify(rows)
    except MySQLError as e:
        app.logger.error(f"Database error: {e}")
        return jsonify({"error": "Failed to fetch data"}), 500
    except Exception as e:
        app.logger.error(f"Unexpected error: {e}")
        return jsonify({"error": "An unexpected error occurred"}), 500
    finally:
        if cursor:
            cursor.close()
        if conn:
            conn.close()


@api_bp.route('/api/insert_data', methods=['POST'])
def insert_data():
    conn = None
    cursor = None
    try:
        user_data = request.json
        # user = User(**user_data)
        conn = get_pool().get_connection()
        cursor = conn.cursor(dictionary=True)
        insert_query = """
        INSERT INTO UserData (user_id, remote_pc, rdp_start_time, rdp_end_time)
        VALUES (%s, %s, %s, %s)
        """
        cursor.execute(insert_query, (
            user_data['id'],
            user_data['remote_pc'],
            user_data['rdp_start_time'],
            user_data['rdp_end_time'],
        ))
        conn.commit()
        return jsonify({"success":"Data inserted successfully"}), 200
    except MySQLError as err:
        print(f"Error: {err}")
        return jsonify({"error": str(err)}), 500
    finally:
        if cursor:
            cursor.close()
        if conn:
            conn.close()


@api_bp.route('/api/update_data', methods=['POST'])
def update_data():
    conn = None
    cursor = None
    try:
        user_data = request.json
        # user = User(**user_data)
        conn = get_pool().get_connection()
        cursor = conn.cursor(dictionary=True)
        insert_query = """
        UPDATE UserData SET rdp_end_time = %s, rdp_start_time = %s 
        WHERE user_id = %s AND remote_pc = %s AND rdp_start_time = %s
        """
        cursor.execute(insert_query, (
            user_data['rdp_end_time'],
            user_data['rdp_start_time'],
            user_data['id'],
            user_data['remote_pc'],
            user_data['rdp_start_time'],
        ))
        conn.commit()
        return jsonify({"success":"Data updated successfully"}), 200
    except MySQLError as err:
        print(f"Error: {err}")
        return jsonify({"error": str(err)}), 500
    finally:
        if cursor:
            cursor.close()
        if conn:
            conn.close()