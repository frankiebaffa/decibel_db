create table DecibelDb.AlbumTracks
	(
		Id integer not null primary key autoincrement
	,	Album_Id integer not null
	,	Song_Id integer not null
	,	File_Id integer null
	,	TrackNumber integer not null
	,	Version text null
	,	Active integer not null default 1
	,	CreatedDate text not null default current_timestamp
	,	LastEditDate text not null default current_timestamp
	,	foreign key (Album_Id) references Albums (Id)
	,	foreign key (Song_Id) references Songs (Id)
	,	foreign key (File_Id) references Files (Id)
	);
