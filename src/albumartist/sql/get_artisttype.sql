select atype.*
from DecibelDb.AlbumArtists as aartist
join DecibelDb.ArtistTypes as atype
on aartist.ArtistType_Id = atype.Id
and aartist.ArtistType_Id = :artisttype_id
and aartist.Active = 1
and atype.Active = 1;
