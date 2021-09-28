select artist.*
from DecibelDb.AlbumArtists as albumartist
join DecibelDb.Artists as artist
on albumartist.Artist_Id = artist.Id
and albumartist.Artist_Id = :artist_id
and albumartist.Active = 1
and artist.Active = 1;
