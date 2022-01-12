CREATE TABLE Accounts(
id bigserial,
firstname varchar(50) NOT NULL,
lastname varchar(50) NOT NULL,
username varchar(100) NOT NULL,
email varchar(100) NOT NULL,
mobile varchar(15) NOT NULL,
accountnmb varchar(200) PRIMARY KEY,
opening_balance bigint NOT NULL,
current_balance bigint NOT NULL,
password varchar(500) NOT NULL,
ip_address varchar(50) NOT NULL,
isactive boolean NULL ,
created_at TIMESTAMP default NOW() NULL,
created_by varchar(20) NULL,
updated_at TIMESTAMP default NOW() NULL,
updated_by varchar(20) NULL
);


CREATE TABLE transactions (
id bigserial PRIMARY KEY,
accountnmb varchar(200), 
recepient_accnt_nmb varchar(200) NOT NULL, 
recepient_name varchar(50) NOT NULL,
amount bigint NOT NULL,
trans_type varchar(50) NOT NULL,
trans_mode varchar(50) NOT NULL,
createdat timestamp DEFAULT now(), 
updatedat timestamp DEFAULT now(),
CONSTRAINT fk_customer
      FOREIGN KEY(accountnmb) 
	  REFERENCES accounts(accountnmb)
);