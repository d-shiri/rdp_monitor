from flask import Flask, request, jsonify
from mysql.connector import pooling, Error as MySQLError
from dotenv import load_dotenv
from pydantic import BaseModel, ValidationError

from funcs import set_tables
from api import api_bp
from database import init_pool

load_dotenv()

app = Flask(__name__)
init_pool()
app.register_blueprint(api_bp)
set_tables()

if __name__ == '__main__':
    set_tables()
    print("Registered routes:")
    for rule in app.url_map.iter_rules():
        print(f"{rule.endpoint}: {rule.rule}")
    # Bind to all interfaces and run in debug mode
    app.run(host='0.0.0.0', port=5000, debug=True)

