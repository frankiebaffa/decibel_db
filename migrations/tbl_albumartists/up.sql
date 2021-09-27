create table DecibelDb.AlbumArtists
	(
		Id integer not null primary key autoincrement
	,	Artist_Id integer not null
	,	Album_Id integer not null
	,	ArtistType_Id integer not null
	,	Active integer not null default 1
	,	CreatedDate text not null default current_timestamp
	,	LastEditDate text not null default current_timestamp
	,	foreign key (Artist_Id) references Artists (Id)
	,	foreign key (Album_Id) references Albums (Id)
	, 	foreign key (ArtistType_Id) references ArtistTypes (Id)
	);
