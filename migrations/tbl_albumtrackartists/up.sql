create table DecibelDb.AlbumTrackArtists
	(
		Id integer not null primary key autoincrement
	,	Artist_Id integer not null
	,	AlbumTrack_Id integer not null
	,	ArtistType_Id integer not null
	,	Active integer not null default 1
	,	CreatedDate text not null
	,	LastEditDate text not null
	,	foreign key (Artist_Id) references Artists (Id)
	,	foreign key (AlbumTrack_Id) references AlbumTracks (Id)
	,	foreign key (ArtistType_Id) references ArtistTypes (Id)
	);
