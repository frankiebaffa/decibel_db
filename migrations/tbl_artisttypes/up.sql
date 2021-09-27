create table BangersDb.ArtistTypes
	(
		Id integer not null primary key autoincrement
	,	Name text not null
	,	Descriptor text not null
	,	Description text null
	,	Active integer not null default 1
	,	CreatedDate text not null default current_timestamp
	,	LastEditDate text not null default current_timestamp
	);
insert into BangersDb.ArtistTypes
	(
		Name
	,	Descriptor
	)
select 'Performer'
,	'Performed by'
union all
select 'Writer'
,	'Written by'
union all
select 'Composer'
,	'Composed by'
union all
select 'Producer'
,	'Produced by'
union all
select 'Feature'
,	'Featuring'

