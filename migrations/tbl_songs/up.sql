create table DecibelDb.Songs
	(
		Id integer not null primary key autoincrement
	,	Name text not null
	,	Blurb text null
	,	Active integer not null default 1
	,	CreatedDate text not null
	,	LastEditDate text not null
	);
