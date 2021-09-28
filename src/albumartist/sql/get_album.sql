select album.*
from DecibelDb.AlbumArtists as aartist
join DecibelDb.Albums as album
on aartist.Album_Id = album.Id
and aartist.Album_Id = :album_id
and aartist.Active = 1
and album.Active = 1;
