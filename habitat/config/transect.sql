

create table IF NOT EXISTS transect(
   id VARCHAR(255) NOT NULL,
   bearing int NOT NULL,
   start_date datetime  NOT NULL,
   end_date datetime  NOT NULL,
   start_lat double NOT NULL,
   start_lon double NOT NULL,
   end_lat double NOT NULL,
   end_lon double NOT NULL,
   vessel_id varchar(255) NOT NULL,
   observer1_id varchar(255) NOT NULL,
   observer2_id varchar(255),
   PRIMARY KEY ( id )
);
