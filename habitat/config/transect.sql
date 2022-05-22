

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


create table IF NOT EXISTS observation(
   id VARCHAR(255) NOT NULL,
   transect_id VARCHAR(255) NOT NULL,
   obs_type VARCHAR(255) NOT NULL,
   date datetime  NOT NULL,
   bearing int,
   count int,
   lat double,
   lon double,
   distance_km double,
   group_type VARCHAR(255),
   beaufort_type VARCHAR(255),
   weather_type VARCHAR(255),
   PRIMARY KEY ( id )
);
