bind = "0.0.0.0:5000"  # Listen on all network interfaces (internal use only)
workers = 2            # Number of worker processes
threads = 2            # Number of threads per worker

# Logging settings
loglevel = "debug"  # Options: debug, info, warning, error, critical
errorlog = "/srv/nct/logs/error.log"  # Path for error logs
accesslog = "/srv/nct/logs/access.log"  # Path for access logs
# Enable output for access and error logs to stdout
capture_output = True