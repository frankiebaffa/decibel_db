select aartist.*
from DecibelDb.AlbumArtists as aartist
where aartist.Album_Id = :album_id;
