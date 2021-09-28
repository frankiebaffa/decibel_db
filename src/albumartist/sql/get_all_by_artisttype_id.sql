select aartist.*
from DecibelDb.AlbumArtists as aartist
where aartist.ArtistType_Id = :artisttype_id;
