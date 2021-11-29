create table DecibelDb.AlbumTypes
	(
		Id integer not null primary key autoincrement
	,	Name text not null
	,	Description text null
	,	Active integer not null default 1
	);
