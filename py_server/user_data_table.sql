CREATE TABLE UserData ( 
	user_id VARCHAR(255), 
	remote_pc INT, 
	rdp_start_time TIMESTAMP, 
	rdp_end_time TIMESTAMP,
	FOREIGN KEY (user_id) REFERENCES Users(id)
);
