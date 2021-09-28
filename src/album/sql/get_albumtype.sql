select altype.*
from DecibelDb.Albums as album
join DecibelDb.AlbumTypes as altype
on album.AlbumType_Id = altype.Id
and album.AlbumType_Id = :albumtype_id;
