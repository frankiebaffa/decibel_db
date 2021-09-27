select count(name)
from DecibelDb.sqlite_master
where type = 'index'
and name = 'AlbumTrackArtistsUnique';
