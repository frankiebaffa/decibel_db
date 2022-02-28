create table DecibelDb.Files
	(
		Id integer not null primary key autoincrement
	,	FileBlob blob not null
	,	MimeType text not null
	,	Active integer not null default 1
	,	CreatedDate text not null
	,	LastEditDate text not null
	);
