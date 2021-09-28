select albumartist.*
from DecibelDb.AlbumArtists as albumartist
where albumartist.Artist_Id = :artist_id;
