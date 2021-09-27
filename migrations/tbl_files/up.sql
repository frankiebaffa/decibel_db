create table DecibelDb.Files
	(
		Id integer not null primary key autoincrement
	,	FileBlob blob not null
	,	Active integer not null default 1
	,	CreatedDate text not null default current_timestamp
	,	LastEditDate text not null default current_timestamp
	);
