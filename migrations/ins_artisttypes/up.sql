insert into DecibelDb.ArtistTypes
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
,	'Featuring';
