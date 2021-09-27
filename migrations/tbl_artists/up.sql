create table BangersDb.Artists
	(
		Id integer not null primary key autoincrement
	,	Name text not null
	,	Bio text null
	,	Active integer not null default 1
	,	CreatedDate text not null default current_timestamp
	,	LastEditDate text not null default current_timestamp
	);