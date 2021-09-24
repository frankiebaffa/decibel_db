select count(name)
from BangersDb.sqlite_master
where type = 'index'
and name = 'AlbumTypesNameUnique';
