insert into DecibelDb.AlbumTypes
	(
		Name
	,	Description
	)
select 'LP'
,	'A long play'
union all
select 'EP'
,	'An extended play'
union all
select 'Single'
,	'A single release';
select 'Mixtape'
,	'An unofficial release'
union all
select 'Compilation'
,	'A release with varying track artists';
