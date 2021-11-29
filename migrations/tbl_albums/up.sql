create table DecibelDb.Albums
	(
		Id integer not null primary key autoincrement
	,	AlbumType_Id integer not null
	,	Name text not null
	,	Blurb text null
	,	Active integer not null default 1
	,	ReleaseDate text null
	,	CreatedDate text not null
	,	LastEditDate text not null
	,	foreign key (AlbumType_Id) references AlbumTypes(Id)
	);
